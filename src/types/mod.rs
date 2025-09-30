use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct SystemData {
    pub total_memory: f64,
    pub used_memory: f64,
    pub available_memory: f64,
    pub usage_memory: f64,
    pub cpu: CpuInfo,
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

#[derive(Debug, Clone, Default)]
pub struct CpuInfo {
    pub len: usize,
    pub frequency: u64,
    pub brand: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Status {
    Active,
    Todo,
    Done,
    Cancelled,
}

impl Status {
    pub fn get_color(&self) -> Color {
        match self {
            self::Status::Active => Color::Yellow,
            self::Status::Todo => Color::Red,
            self::Status::Done => Color::Green,
            self::Status::Cancelled => Color::DarkGray,
        }
    }
}

//TODO: Добавить дату создания и дату окончания
#[derive(Debug)]
pub struct TODOData {
    pub id: i64,
    pub title: &'static str,
    pub message: &'static str,
    pub status: Status,
}

#[derive(Debug)]
pub struct ListState {
    pub selected: Option<usize>,
    pub items: Vec<TODOData>,
}
