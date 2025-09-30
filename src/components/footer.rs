use ratatui::{
    Frame,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

pub fn render_footer(frame: &mut Frame, area: ratatui::layout::Rect) {
    let title = Line::from("Press `Esc`, `Ctrl-C` or `q` to stop running.")
        .bold()
        .white()
        .centered();

    let block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::White))
        .title(title);

    frame.render_widget(Paragraph::new("").block(block).centered(), area);
}
