use chrono::{DateTime, Local};
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
    Todo,
    Active,
    Done,
    Cancelled,
}

impl Status {
    pub fn get_color(&self) -> Color {
        match self {
            self::Status::Active => Color::Yellow,
            self::Status::Todo => Color::Red,
            self::Status::Done => Color::Green,
            self::Status::Cancelled => Color::Gray,
        }
    }
}

//TODO: Добавить дату создания и дату окончания
#[derive(Debug)]
pub struct TODOData {
    pub id: i64,
    pub title: &'static str,
    pub message: &'static str,
    pub date: DateTime<Local>,
    pub status: Status,
}

impl TODOData {
    pub fn toggle_status(&mut self) -> Status {
        self.status = match self.status {
            Status::Todo => Status::Active,
            Status::Active => Status::Done,
            Status::Done => Status::Cancelled,
            Status::Cancelled => Status::Todo,
        };
        self.status
    }
}

#[derive(Debug)]
pub struct ListState {
    pub selected: Option<usize>,
    pub items: Vec<TODOData>,
}

impl Default for ListState {
    fn default() -> Self {
        Self::new()
    }
}

impl ListState {
    pub fn new() -> Self {
        Self {
            selected: Some(0),
            items: Vec::new(),
        }
    }

    pub fn next(&mut self) {
        let len = self.items.len();
        if len == 0 {
            return;
        }

        let i = match self.selected {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    pub fn previous(&mut self) {
        let len = self.items.len();
        if len == 0 {
            return;
        }

        let i = match self.selected {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    pub fn selected_item(&self) -> Option<&TODOData> {
        self.selected.and_then(|i| self.items.get(i))
    }
}
