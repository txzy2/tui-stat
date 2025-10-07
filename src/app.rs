use std::{
    fmt,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputField {
    Title,
    Message,
}

use crossterm::event;
use ratatui::text::Text;
use reqwest::Client;
use rusqlite::{Connection, Result};
use tokio::{
    runtime::Handle,
    sync::mpsc::{self, error::TryRecvError, UnboundedReceiver, UnboundedSender},
};

use crate::{
    components, logger,
    system::{keys_handler, system_info::System},
    types::{GeoData, ListState, SystemData, TODOData, WeatherInfo, WeatherResponse},
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
    pub sys_data: SystemData,
    pub sys_collector: System,
    pub sys_text: Text<'static>,
    pub animation_frame: usize,
    client: Client,
    runtime: Handle,
    updates_rx: UnboundedReceiver<AsyncUpdate>,
    updates_tx: UnboundedSender<AsyncUpdate>,
    pub list_state: ListState,
    pub show_item: bool,
    pub show_help: bool,
    db_connection: Option<Connection>,
    // Input state for adding new TODO
    pub show_add_modal: bool,
    pub input_title: String,
    pub input_message: String,
    pub input_cursor_pos: usize,
    pub input_current_field: InputField, // 0 for title, 1 for message
}

impl fmt::Debug for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("running", &self.running)
            .field("ip", &self.ip)
            .field("geo_data", &self.geo_data)
            .field("weather_data", &self.weather_data)
            .field("show_quit_modal", &self.show_quit_modal)
            .field("sys_data", &self.sys_data)
            .finish()
    }
}

impl App {
    pub fn new(runtime: Handle) -> Self {
        let client = Client::new();
        let (updates_tx, updates_rx) = mpsc::unbounded_channel();
        let mut sys_collector = System::new();
        let sys_data = sys_collector.get_info();
        let sys_text = components::format_sys_text(&sys_data);

        // Initialize database connection
        let db_connection = match Self::open_sqlite_con("data.db") {
            Ok(conn) => {
                // Create the todos table if it doesn't exist
                if let Err(e) = conn.execute(
                    "CREATE TABLE IF NOT EXISTS todos (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        title TEXT NOT NULL,
                        message TEXT,
                        status TEXT NOT NULL,
                        date TEXT NOT NULL
                    )",
                    [],
                ) {
                    eprintln!("Error creating table: {}", e);
                }
                Some(conn)
            }
            Err(e) => {
                eprintln!("Error opening database: {}", e);
                None
            }
        };

        let mut app = Self {
            running: true,
            ip: String::new(),
            geo_data: None,
            weather_data: None,
            show_quit_modal: false,
            sys_data,
            sys_collector,
            sys_text,
            animation_frame: 0,
            client,
            runtime,
            updates_rx,
            updates_tx,
            list_state: ListState::new(),
            show_item: false,
            show_help: false,
            db_connection,
            show_add_modal: false,
            input_title: String::new(),
            input_message: String::new(),
            input_cursor_pos: 0,
            input_current_field: InputField::Title,
        };

        // Load todos from database after initialization
        if let Err(e) = app.load_todos_from_db() {
            eprintln!("Error loading todos from database: {}", e);
        }

        app
    }

    fn open_sqlite_con(db_name: &str) -> Result<Connection, rusqlite::Error> {
        Connection::open(db_name)
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
                self.animation_frame = (self.animation_frame + 1) % 4;
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

    pub fn load_todos_from_db(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref conn) = self.db_connection {
            let mut stmt = conn.prepare(
                "
                    SELECT id, title, message, status, date 
                    FROM todos ORDER BY CASE status
                    WHEN 'Active' THEN 1 WHEN 'Todo' THEN 2 WHEN 'Cancelled' THEN 3 WHEN 'Done' THEN 4 ELSE 5 END, id
                ",
            )?;

            let todo_iter = stmt.query_map([], |row| {
                let id: i64 = row.get(0)?;
                let title: String = row.get(1)?;
                let message: String = row.get(2)?;
                let status_str: String = row.get(3)?;
                let date_str: String = row.get(4)?;

                let status = match status_str.as_str() {
                    "Todo" => crate::types::Status::Todo,
                    "Active" => crate::types::Status::Active,
                    "Done" => crate::types::Status::Done,
                    "Cancelled" => crate::types::Status::Cancelled,
                    _ => crate::types::Status::Todo, // default
                };

                let date = chrono::DateTime::parse_from_rfc3339(&date_str)
                    .unwrap_or_else(|_| chrono::Local::now().into())
                    .with_timezone(&chrono::Local);

                // Convert String to &'static str by leaking the memory (not ideal but needed for the current struct design)
                Ok(TODOData {
                    id,
                    title: Box::leak(title.into_boxed_str()),
                    message: Box::leak(message.into_boxed_str()),
                    status,
                    date,
                })
            })?;

            let mut items = Vec::new();
            for todo in todo_iter.flatten() {
                items.push(todo);
            }

            // Update the list state with loaded items
            self.list_state.items = items;

            // Reset selection if needed
            if !self.list_state.items.is_empty() {
                self.list_state.selected = Some(0);
            } else {
                self.list_state.selected = None;
            }
        }
        Ok(())
    }

    pub fn add_todo_to_db(
        &self,
        title: &str,
        message: &str,
        status: crate::types::Status,
    ) -> Result<i64> {
        if let Some(ref conn) = self.db_connection {
            let status_str = match status {
                crate::types::Status::Todo => "Todo",
                crate::types::Status::Active => "Active",
                crate::types::Status::Done => "Done",
                crate::types::Status::Cancelled => "Cancelled",
            };

            let date_str = chrono::Local::now().to_rfc3339();

            conn.execute(
                "INSERT INTO todos (title, message, status, date) VALUES (?1, ?2, ?3, ?4)",
                [title, message, status_str, &date_str],
            )?;

            Ok(conn.last_insert_rowid())
        } else {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1),
                Some("Database connection not available".to_string()),
            ))
        }
    }

    pub fn update_todo_status_in_db(
        &mut self,
        id: i64,
        status: crate::types::Status,
    ) -> Result<()> {
        if let Some(ref conn) = self.db_connection {
            let status_str = match status {
                crate::types::Status::Todo => "Todo",
                crate::types::Status::Active => "Active",
                crate::types::Status::Done => "Done",
                crate::types::Status::Cancelled => "Cancelled",
            };

            conn.execute(
                "UPDATE todos SET status = ?1 WHERE id = ?2",
                [status_str, &id.to_string()],
            )?;

            Ok(())
        } else {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1),
                Some("Database connection not available".to_string()),
            ))
        }
    }

    pub fn delete_todo_from_db(&self, id: i64) -> Result<()> {
        if let Some(ref conn) = self.db_connection {
            conn.execute("DELETE FROM todos WHERE id = ?1", [id])?;
            Ok(())
        } else {
            Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1),
                Some("Database connection not available".to_string()),
            ))
        }
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
        let updated = self.sys_collector.get_info();
        self.sys_text = components::format_sys_text(&updated);
        self.sys_data = updated;
    }

    fn handle_events(&mut self) -> color_eyre::Result<()> {
        keys_handler::KeyHandler::handle_crossterm_events(self)?;
        Ok(())
    }

    pub(crate) fn request_quit(&mut self) {
        self.running = false;
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
