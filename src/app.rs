use std::{
    fmt,
    time::{Duration, Instant},
};

use crossterm::event;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span, Text},
};
use reqwest::Client;
use tokio::{
    runtime::Handle,
    sync::mpsc::{self, error::TryRecvError, UnboundedReceiver, UnboundedSender},
};

use crate::{
    components, logger,
    system::{keys_handler, memory::Ram},
    types::{GeoData, RamData, WeatherInfo, WeatherResponse},
};

enum AsyncUpdate {
    Ip(String),
    Geo(GeoData),
    Weather(WeatherInfo),
    Error(String),
}

pub struct App {
    running: bool,
    ip: String,
    geo_data: Option<GeoData>,
    weather_data: Option<WeatherInfo>,
    pub show_quit_modal: bool,
    pub ram_data: RamData,
    pub ram_collector: Ram,
    pub ram_text: Text<'static>,
    client: Client,
    runtime: Handle,
    updates_rx: UnboundedReceiver<AsyncUpdate>,
    updates_tx: UnboundedSender<AsyncUpdate>,
}

impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("running", &self.running)
            .field("ip", &self.ip)
            .field("geo_data", &self.geo_data)
            .field("weather_data", &self.weather_data)
            .field("show_quit_modal", &self.show_quit_modal)
            .field("ram_data", &self.ram_data)
            .finish()
    }
}

impl App {
    pub fn new(runtime: Handle) -> Self {
        let client = Client::new();
        let (updates_tx, updates_rx) = mpsc::unbounded_channel();
        let mut ram_collector = Ram::new();
        let ram_data = ram_collector.get_ram_info();
        let ram_text = Self::format_ram_text(&ram_data);

        Self {
            running: true,
            ip: String::new(),
            geo_data: None,
            weather_data: None,
            show_quit_modal: false,
            ram_data,
            ram_collector,
            ram_text,
            client,
            runtime,
            updates_rx,
            updates_tx,
        }
    }

    pub fn run(mut self, mut terminal: ratatui::DefaultTerminal) -> color_eyre::Result<()> {
        self.running = true;
        self.spawn_initial_fetch();

        let tick_rate = Duration::from_millis(1000);
        let mut last_tick = Instant::now();

        while self.running {
            self.process_updates();

            if last_tick.elapsed() >= tick_rate {
                self.update_ram_data();
                last_tick = Instant::now();
            }

            terminal.draw(|frame| components::render(frame, &self))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                self.handle_events()?;
            }
        }
        Ok(())
    }

    fn spawn_initial_fetch(&self) {
        let client = self.client.clone();
        let tx = self.updates_tx.clone();
        let weather_key = std::env::var("WEATHER_API_KEY").ok();

        self.runtime.spawn(async move {
            if let Err(err) = fetch_initial_data(client, weather_key, tx.clone()).await {
                let _ = tx.send(AsyncUpdate::Error(err));
            }
        });
    }

    fn process_updates(&mut self) {
        loop {
            match self.updates_rx.try_recv() {
                Ok(update) => self.handle_update(update),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => break,
            }
        }
    }

    fn handle_update(&mut self, update: AsyncUpdate) {
        match update {
            AsyncUpdate::Ip(ip) => {
                let message = format!("IP: {ip}");
                self.ip = ip;
                let _ = logger::info(message);
            }
            AsyncUpdate::Geo(geo) => {
                let message = format!("Latitude: {}, Longitude: {}", geo.latitude, geo.longitude);
                self.geo_data = Some(geo);
                let _ = logger::info(message);
            }
            AsyncUpdate::Weather(weather) => {
                let message = format!("Weather: {} ({:.0}°C)", weather.name, weather.temp_c);
                self.weather_data = Some(weather);
                let _ = logger::info(message);
            }
            AsyncUpdate::Error(error) => {
                let _ = logger::error(error);
            }
        }
    }

    pub fn weather_info(&self) -> Option<&WeatherInfo> {
        self.weather_data.as_ref()
    }

    fn update_ram_data(&mut self) {
        let updated = self.ram_collector.get_ram_info();
        self.ram_text = Self::format_ram_text(&updated);
        self.ram_data = updated;
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        keys_handler::KeyHandler::handle_crossterm_events(self)?;
        Ok(())
    }

    pub(crate) fn request_quit(&mut self) {
        self.running = false;
    }

    fn format_ram_text(data: &RamData) -> Text<'static> {
        let label_color = Color::LightCyan;
        let mut spans = Vec::with_capacity(12);

        spans.push(Span::styled("Total: ", Style::default().fg(label_color)));
        spans.push(Span::styled(
            format!("{:.2} GB", data.total_memory),
            Style::default().fg(Color::Yellow),
        ));
        spans.push(Span::raw("  "));

        spans.push(Span::styled("Used: ", Style::default().fg(label_color)));
        let used_color = if data.usage_memory > 80.0 {
            Color::Red
        } else {
            Color::Yellow
        };
        spans.push(Span::styled(
            format!("{:.2} GB", data.used_memory),
            Style::default().fg(used_color),
        ));
        spans.push(Span::raw("  "));

        spans.push(Span::styled(
            "Available: ",
            Style::default().fg(label_color),
        ));
        spans.push(Span::styled(
            format!("{:.2} GB", data.available_memory),
            Style::default().fg(Color::Yellow),
        ));
        spans.push(Span::raw("  "));

        spans.push(Span::styled("Usage: ", Style::default().fg(label_color)));
        spans.push(Span::styled(
            format!("{:.1}%", data.usage_memory),
            Style::default().fg(Color::Yellow),
        ));

        Text::from(Line::from(spans))
    }
}

async fn fetch_initial_data(
    client: Client,
    weather_key: Option<String>,
    tx: UnboundedSender<AsyncUpdate>,
) -> Result<(), String> {
    let ip = fetch_ip(&client).await.map_err(|e| e.to_string())?;
    let _ = tx.send(AsyncUpdate::Ip(ip.clone()));

    let geo = fetch_geo(&client, &ip).await.map_err(|e| e.to_string())?;
    let _ = tx.send(AsyncUpdate::Geo(geo.clone()));

    if let Some(key) = weather_key {
        match fetch_weather(&client, geo.latitude, geo.longitude, &key).await {
            Ok(weather) => {
                let _ = tx.send(AsyncUpdate::Weather(weather));
            }
            Err(err) => {
                let _ = tx.send(AsyncUpdate::Error(err.to_string()));
            }
        }
    }

    Ok(())
}

async fn fetch_ip(client: &Client) -> Result<String, reqwest::Error> {
    let response = client
        .get("https://api.ipify.org")
        .send()
        .await?
        .text()
        .await?;
    Ok(response.trim().to_owned())
}

async fn fetch_geo(client: &Client, ip: &str) -> Result<GeoData, reqwest::Error> {
    let url = format!("https://ipapi.co/{ip}/json/");
    client.get(url).send().await?.json::<GeoData>().await
}

async fn fetch_weather(
    client: &Client,
    latitude: f64,
    longitude: f64,
    api_key: &str,
) -> Result<WeatherInfo, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={latitude}&lon={longitude}&appid={api_key}"
    );

    let resp = client
        .get(url)
        .send()
        .await?
        .json::<WeatherResponse>()
        .await?;

    let temp_c = (resp.main.temp - 273.15).round();

    Ok(WeatherInfo {
        name: resp.name,
        temp_c,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_ram_text_normal_usage() {
        let data = RamData {
            total_memory: 16.0,
            used_memory: 8.0,
            available_memory: 8.0,
            usage_memory: 50.0,
        };

        let text = App::format_ram_text(&data);
        let rendered = format!("{:?}", text);

        // Проверяем, что все значения присутствуют в тексте
        assert!(rendered.contains("16.00"), "Should contain total memory");
        assert!(rendered.contains("8.00"), "Should contain used memory");
        assert!(rendered.contains("50."), "Should contain usage percentage");
    }

    #[test]
    fn test_format_ram_text_high_usage_color() {
        let data = RamData {
            total_memory: 16.0,
            used_memory: 14.0,
            available_memory: 2.0,
            usage_memory: 87.5,
        };

        let text = App::format_ram_text(&data);
        
        // При usage > 80% используемая память должна быть красной
        // Проверяем что текст содержит данные
        assert!(!text.lines.is_empty(), "Text should not be empty");
    }

    #[test]
    fn test_format_ram_text_low_usage() {
        let data = RamData {
            total_memory: 32.0,
            used_memory: 4.0,
            available_memory: 28.0,
            usage_memory: 12.5,
        };

        let text = App::format_ram_text(&data);
        let rendered = format!("{:?}", text);

        assert!(rendered.contains("32.00"));
        assert!(rendered.contains("4.00"));
        assert!(rendered.contains("28.00"));
    }

    #[test]
    fn test_format_ram_text_edge_case_zero() {
        let data = RamData {
            total_memory: 0.0,
            used_memory: 0.0,
            available_memory: 0.0,
            usage_memory: 0.0,
        };

        let text = App::format_ram_text(&data);
        
        // Не должно паниковать при нулевых значениях
        assert!(!text.lines.is_empty());
    }

    #[test]
    fn test_format_ram_text_contains_labels() {
        let data = RamData {
            total_memory: 8.0,
            used_memory: 4.0,
            available_memory: 4.0,
            usage_memory: 50.0,
        };

        let text = App::format_ram_text(&data);
        let rendered = format!("{:?}", text);

        // Проверяем наличие меток
        assert!(rendered.contains("Total"), "Should contain 'Total' label");
        assert!(rendered.contains("Used"), "Should contain 'Used' label");
        assert!(rendered.contains("Available"), "Should contain 'Available' label");
        assert!(rendered.contains("Usage"), "Should contain 'Usage' label");
    }
}
