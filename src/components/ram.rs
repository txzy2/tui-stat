use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::types::SystemData;

pub fn render_memory_info(frame: &mut Frame, area: Rect, text: &Text<'_>) {
    let title = Line::from("System Info").bold().white().centered();
    let block = Block::default().borders(Borders::NONE).title(title);

    frame.render_widget(
        Paragraph::new(text.clone())
            .alignment(Alignment::Center)
            .block(block),
        area,
    );
}

pub fn format_sys_text(data: &SystemData) -> Text<'static> {
    let label_color = Color::DarkGray;
    let value_color = Color::White;
    let mut lines = Vec::new();

    // CPU info line (первая строка)
    let mut cpu_spans = Vec::new();
    cpu_spans.push(Span::styled("CPU: ", Style::default().fg(label_color)));
    cpu_spans.push(Span::styled(
        data.cpu.brand.clone(),
        Style::default().fg(value_color),
    ));
    cpu_spans.push(Span::raw("  "));

    cpu_spans.push(Span::styled("Cores: ", Style::default().fg(label_color)));
    cpu_spans.push(Span::styled(
        format!("{}", data.cpu.len),
        Style::default().fg(value_color),
    ));
    cpu_spans.push(Span::raw("  "));

    cpu_spans.push(Span::styled(
        "Frequency: ",
        Style::default().fg(label_color),
    ));
    cpu_spans.push(Span::styled(
        format!("{} MHz", data.cpu.frequency),
        Style::default().fg(value_color),
    ));

    lines.push(Line::from(cpu_spans));

    // RAM info line (вторая строка)
    let mut ram_spans = Vec::with_capacity(12);
    ram_spans.push(Span::styled("Total: ", Style::default().fg(label_color)));
    ram_spans.push(Span::styled(
        format!("{:.2} GB", data.total_memory),
        Style::default().fg(value_color),
    ));
    ram_spans.push(Span::raw("  "));

    ram_spans.push(Span::styled("Used: ", Style::default().fg(label_color)));
    let used_color = if data.usage_memory > 80.0 {
        Color::Red
    } else {
        value_color
    };
    ram_spans.push(Span::styled(
        format!("{:.2} GB", data.used_memory),
        Style::default().fg(used_color),
    ));
    ram_spans.push(Span::raw("  "));

    ram_spans.push(Span::styled(
        "Available: ",
        Style::default().fg(label_color),
    ));
    ram_spans.push(Span::styled(
        format!("{:.2} GB", data.available_memory),
        Style::default().fg(value_color),
    ));
    ram_spans.push(Span::raw("  "));

    ram_spans.push(Span::styled("Usage: ", Style::default().fg(label_color)));
    ram_spans.push(Span::styled(
        format!("{:.1}%", data.usage_memory),
        Style::default().fg(value_color),
    ));

    lines.push(Line::from(ram_spans));

    Text::from(lines)
}
