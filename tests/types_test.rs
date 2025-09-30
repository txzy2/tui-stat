use tui_stat::types::{GeoData, RamData, WeatherInfo};

#[test]
fn test_ram_data_structure() {
    let ram_data = RamData {
        total_memory: 16.0,
        used_memory: 8.0,
        available_memory: 8.0,
        usage_memory: 50.0,
    };

    assert_eq!(ram_data.total_memory, 16.0);
    assert_eq!(ram_data.used_memory, 8.0);
    assert_eq!(ram_data.available_memory, 8.0);
    assert_eq!(ram_data.usage_memory, 50.0);
}

#[test]
fn test_ram_data_clone() {
    let ram_data1 = RamData {
        total_memory: 32.0,
        used_memory: 16.0,
        available_memory: 16.0,
        usage_memory: 50.0,
    };

    let ram_data2 = ram_data1.clone();

    assert_eq!(ram_data1.total_memory, ram_data2.total_memory);
    assert_eq!(ram_data1.used_memory, ram_data2.used_memory);
}

#[test]
fn test_geo_data_structure() {
    let geo_data = GeoData {
        latitude: 55.7558,
        longitude: 37.6173,
    };

    assert_eq!(geo_data.latitude, 55.7558);
    assert_eq!(geo_data.longitude, 37.6173);
}

#[test]
fn test_weather_info_structure() {
    let weather = WeatherInfo {
        name: "Moscow".to_string(),
        temp_c: 15.0,
    };

    assert_eq!(weather.name, "Moscow");
    assert_eq!(weather.temp_c, 15.0);
}

#[test]
fn test_weather_info_clone() {
    let weather1 = WeatherInfo {
        name: "London".to_string(),
        temp_c: 12.5,
    };

    let weather2 = weather1.clone();

    assert_eq!(weather1.name, weather2.name);
    assert_eq!(weather1.temp_c, weather2.temp_c);
}
