use chrono::{DateTime, Local};
use ratatui::{
    Frame,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn render_welcome(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let datetime: DateTime<Local> = Local::now();

    let title_text = if let Some(weather) = app.weather_info() {
        format!(
            "TUI DISPATCHER [{}] [{} {:.0}°C]",
            datetime.format("%H:%M:%S"),
            weather.name,
            weather.temp_c
        )
    } else {
        format!(
            "TUI DISPATCHER [{}] [loading weather]",
            datetime.format("%H:%M:%S")
        )
    };

    let title = Line::from(title_text).bold().blue().centered();

    let block = Block::default()
        .borders(Borders::BOTTOM)
        .border_style(Style::default().fg(Color::White))
        .title(title);

    frame.render_widget(Paragraph::new("").block(block).centered(), area);
}
