use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::types::{ListState, TODOData};

pub fn render_select(frame: &mut Frame, area: Rect, item: &ListState) {
    if item.items.is_empty() {
        return;
    }

    let selected_index = match item.selected {
        Some(index) if index < item.items.len() => index,
        _ => {
            return;
        }
    };

    let data: &TODOData = &item.items[selected_index];

    let title = Line::from(vec![
        Span::raw(format!("Title: {}", data.title)),
        Span::styled(
            format!(" ({})", data.date.format("%d.%m.%y %H:%M")),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::DIM),
        ),
    ])
    .bold()
    .white()
    .centered();

    let mut text = Vec::new();
    text.push(Line::from(vec![
        Span::raw("Status: "),
        Span::styled(
            format!("{:?}", data.status),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(data.status.get_color()),
        ),
    ]));
    text.push(Line::from(format!("Description: {}", data.message)));

    let block = Block::default()
        .border_style(Style::default().fg(Color::White))
        .title(title);

    frame.render_widget(Paragraph::new(text).block(block), area);
}
