use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{
    logger,
    types::{ListState, TODOData},
};

pub fn render_select(frame: &mut Frame, area: Rect, item: &ListState) {
    let data: &TODOData = &item.items[item.selected.unwrap()];
    let _ = logger::info(format!("selected item {:?}", data));

    //TODO: Сделать после добавления даты в заголовке отображение спарва
    let title = Line::from(format!("Title: {}", data.title))
        .bold()
        .white()
        .centered();

    let mut text = Vec::new();
    text.push(
        Line::from(format!("Status: {:?}", data.status))
            .style(Style::default().fg(data.status.get_color())),
    );
    text.push(Line::from(format!("Description: {}", data.message)));

    let block = Block::default()
        .border_style(Style::default().fg(Color::White))
        .title(title);

    frame.render_widget(Paragraph::new(text).block(block), area);
}
