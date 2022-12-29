mod weather;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let city_name = "Sydney";
    let result = weather::get_city_geo_location(city_name);
    let (lat, long) = result;
    let (current_temp, max_temp, min_temp, weather_description) =
        weather::get_current_weather(&lat, &long);
    println!(
        "
Today's Weather in {} is {}\n
Temperature: {} \n
Max: {} \n
Min: {}
    ",
        city_name,
        weather_description.as_str().unwrap(),
        current_temp,
        max_temp,
        min_temp
    );

    let timestamp_results = weather::get_timestamps_weather_forecasting(&lat, &long);

    for result in timestamp_results {
        println!(
            "
Datetime: {} \n 
Weather in {} is {}\n
Temperature: {} \n
Max: {} \n
Min: {}
        ",
            result.datetime.format("%Y-%m-%d %H:%M"),
            city_name,
            result.weather_description,
            result.current_temperature,
            result.max_temperature,
            result.min_temperature
        );
    }
    Ok(())
}
