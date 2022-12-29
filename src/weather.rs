use chrono::{DateTime, Local, TimeZone};
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref WEATHER_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub struct FormattedResult {
    pub datetime: DateTime<Local>,
    pub current_temperature: String,
    pub max_temperature: String,
    pub min_temperature: String,
    pub weather_description: String,
}

struct WeatherClient {
    api_key: String,
}

impl Default for WeatherClient {
    fn default() -> WeatherClient {
        WeatherClient {
            api_key: env::var("API_KEY").unwrap().to_string(),
        }
    }
}

impl WeatherClient {
    /**
     * Async get request and return json response in `serde_json::Value`
     */
    #[tokio::main]
    async fn get(&self, url: &str) -> serde_json::Value {
        let weather_url = format!("{}&appid={}", url, self.api_key);
        let text_response = WEATHER_CLIENT
            .get(weather_url)
            .send()
            .await
            .expect("Request error")
            .text()
            .await
            .unwrap_or_default();

        let json_response: serde_json::Value =
            serde_json::from_str(&text_response).unwrap_or_default();
        json_response
    }
}

/**
* Get latitude and longtitude based on city name.
*/
pub fn get_city_geo_location(city_name: &str) -> (serde_json::Value, serde_json::Value) {
    let geolocation_url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=5",
        city_name,
    );
    let client = WeatherClient::default();
    let json_response = client.get(&geolocation_url);

    let latitude = &json_response[0]["lat"];
    let longitude = &json_response[0]["lon"];
    (latitude.clone(), longitude.clone())
}

/**
Get weather information in the current day and expected return the following
- current_temperature
- max_temperature
- min_temperature
- weather_description
*/
pub fn get_current_weather(
    lat: &serde_json::Value,
    lon: &serde_json::Value,
) -> (
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
    serde_json::Value,
) {
    let weather_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units=metric",
        *lat, *lon,
    );
    let client = WeatherClient::default();
    let json_response = client.get(&weather_url);

    let main_info = &json_response["main"];
    let current_temperature = &main_info["temp"];
    let max_temperature = &main_info["temp_max"];
    let min_temperature = &main_info["temp_min"];
    let weather_description = &json_response["weather"][0]["description"];
    (
        current_temperature.to_owned(),
        max_temperature.to_owned(),
        min_temperature.to_owned(),
        weather_description.to_owned(),
    )
}

/**
Get weather information in the serval days and expected to return
the list of following information:
- datetime
- current_temperature
- max_temperature
- min_temperature
- weather_description
*/
pub fn get_timestamps_weather_forecasting(
    lat: &serde_json::Value,
    lon: &serde_json::Value,
) -> Vec<FormattedResult> {
    let weather_url = format!(
        "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&cnt=7&units=metric",
        *lat, *lon,
    );
    let client = WeatherClient::default();
    let json_response = client.get(&weather_url);
    let forcasting_results = json_response["list"].as_array().unwrap();

    let mut formatted_result = vec![];

    for result in forcasting_results {
        let raw_time = result["dt"].as_i64().unwrap();
        let formatted_time = Local.timestamp_opt(raw_time, 0).unwrap();

        let main_info = &result["main"];
        let current_temperature = &main_info["temp"].to_owned();
        let max_temperature = &main_info["temp_max"].to_owned();
        let min_temperature = &main_info["temp_min"].to_owned();
        let weather_description = &result["weather"][0]["description"].to_owned();

        formatted_result.push(FormattedResult {
            datetime: formatted_time,
            current_temperature: current_temperature.to_string(),
            max_temperature: max_temperature.to_string(),
            min_temperature: min_temperature.to_string(),
            weather_description: weather_description.to_string(),
        });
    }

    formatted_result
}

#[cfg(test)]
mod tests {
    use super::*;
}
