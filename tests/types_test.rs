use tui_stat::types::{CpuInfo, GeoData, SystemData, WeatherInfo};

#[test]
fn test_system_data_structure() {
    let cpu_info = CpuInfo {
        len: 8,
        frequency: 3600,
        brand: "Intel Core i7".to_string(),
    };

    let system_data = SystemData {
        total_memory: 16.0,
        used_memory: 8.0,
        available_memory: 8.0,
        usage_memory: 50.0,
        cpu: cpu_info,
    };

    assert_eq!(system_data.total_memory, 16.0);
    assert_eq!(system_data.used_memory, 8.0);
    assert_eq!(system_data.available_memory, 8.0);
    assert_eq!(system_data.usage_memory, 50.0);
    assert_eq!(system_data.cpu.len, 8);
    assert_eq!(system_data.cpu.frequency, 3600);
}

#[test]
fn test_system_data_clone() {
    let cpu_info = CpuInfo {
        len: 4,
        frequency: 2400,
        brand: "AMD Ryzen 5".to_string(),
    };

    let system_data1 = SystemData {
        total_memory: 32.0,
        used_memory: 16.0,
        available_memory: 16.0,
        usage_memory: 50.0,
        cpu: cpu_info,
    };

    let system_data2 = system_data1.clone();

    assert_eq!(system_data1.total_memory, system_data2.total_memory);
    assert_eq!(system_data1.used_memory, system_data2.used_memory);
    assert_eq!(system_data1.cpu.len, system_data2.cpu.len);
    assert_eq!(system_data1.cpu.brand, system_data2.cpu.brand);
}

#[test]
fn test_cpu_info_structure() {
    let cpu = CpuInfo {
        len: 16,
        frequency: 4800,
        brand: "AMD Ryzen 9".to_string(),
    };

    assert_eq!(cpu.len, 16);
    assert_eq!(cpu.frequency, 4800);
    assert_eq!(cpu.brand, "AMD Ryzen 9");
}

#[test]
fn test_cpu_info_default() {
    let cpu = CpuInfo::default();

    assert_eq!(cpu.len, 0);
    assert_eq!(cpu.frequency, 0);
    assert_eq!(cpu.brand, "");
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
