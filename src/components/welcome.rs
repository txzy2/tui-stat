use chrono::{DateTime, Local};
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

fn get_loading_animation(frame: usize) -> &'static str {
    match frame % 4 {
        0 => "",
        1 => ".",
        2 => "..",
        3 => "...",
        _ => "",
    }
}

pub fn render_welcome(frame: &mut Frame, area: ratatui::layout::Rect, app: &App) {
    let datetime: DateTime<Local> = Local::now();
    let label_color = Color::DarkGray;
    let value_color = Color::White;

    let title_line = if let Some(weather) = app.weather_info() {
        Line::from(vec![
            Span::styled("TUITASK ", Style::default().fg(value_color).bold()),
            Span::styled("[", Style::default().fg(label_color)),
            Span::styled(
                datetime.format("%H:%M:%S").to_string(),
                Style::default().fg(value_color),
            ),
            Span::styled("] [", Style::default().fg(label_color)),
            Span::styled(&weather.name, Style::default().fg(value_color)),
            Span::styled(" ", Style::default()),
            Span::styled(
                format!("{:.0}°C", weather.temp_c),
                Style::default().fg(value_color),
            ),
            Span::styled("]", Style::default().fg(label_color)),
        ])
    } else {
        let dots = get_loading_animation(app.animation_frame);
        Line::from(vec![
            Span::styled("TUI DISPATCHER ", Style::default().fg(label_color).bold()),
            Span::styled("[", Style::default().fg(label_color)),
            Span::styled(
                datetime.format("%H:%M:%S").to_string(),
                Style::default().fg(value_color),
            ),
            Span::styled("] [", Style::default().fg(label_color)),
            Span::styled("Loading ", Style::default().fg(value_color)),
            Span::styled(dots, Style::default().fg(value_color)),
            Span::styled("]", Style::default().fg(label_color)),
        ])
    };

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::White))
        .title(title_line.centered());

    frame.render_widget(Paragraph::new("").block(block).centered(), area);
}
