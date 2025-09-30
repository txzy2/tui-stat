use ratatui::{
    layout::{Alignment, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_memory_info(frame: &mut Frame, area: Rect, text: &Text<'_>) {
    let title = Line::from("Ram Info").bold().white().centered();
    let block = Block::default().borders(Borders::NONE).title(title);

    frame.render_widget(
        Paragraph::new(text.clone())
            .alignment(Alignment::Center)
            .block(block),
        area,
    );
}
