use ratatui::{
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_main_right(frame: &mut Frame, area: ratatui::layout::Rect) {
    let title = Line::from("Main Right").bold().white().centered();

    let block = Block::default()
        .border_style(Style::default().fg(Color::White))
        .borders(Borders::RIGHT)
        .title(title);

    frame.render_widget(Paragraph::new("").block(block).centered(), area);
}
