use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct RamData {
    pub total_memory: f64,
    pub used_memory: f64,
    pub available_memory: f64,
    pub usage_memory: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GeoData {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WeatherResponse {
    pub name: String,
    pub main: WeatherMain,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WeatherMain {
    pub temp: f64,
}

#[derive(Debug, Clone)]
pub struct WeatherInfo {
    pub name: String,
    pub temp_c: f64,
}
