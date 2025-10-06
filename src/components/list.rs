use ratatui::{
    prelude::Stylize,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::types::ListState;

pub fn render_list(frame: &mut Frame, area: ratatui::layout::Rect, state: &ListState) {
    let title = Line::from(vec![Span::raw("[== TODOS PANEL ==]")])
        .bold()
        .centered();

    let items: Vec<ListItem> = if state.items.is_empty() {
        vec![ListItem::new(
            Line::from(Span::styled(
                "Add some =]",
                Style::default().fg(Color::DarkGray),
            ))
            .centered(),
        )]
    } else {
        state
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let status_str = format!("[{:?}]", item.status);
                let padded_status = format!("{:12}", status_str);

                let content = if state.selected == Some(i) {
                    Line::from(vec![
                        Span::styled(
                            format!("  ● {} ", padded_status),
                            Style::default()
                                .fg(Color::Rgb(255, 203, 164))
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("{} ", item.title),
                            Style::default()
                                .fg(Color::Rgb(255, 203, 164))
                                .add_modifier(Modifier::ITALIC),
                        ),
                    ])
                } else {
                    Line::from(vec![
                        Span::styled(
                            format!("  ○ {} ", padded_status),
                            Style::default().fg(match item.status {
                                crate::types::Status::Done => Color::Rgb(80, 80, 80),
                                _ => Color::DarkGray,
                            }),
                        ),
                        Span::styled(format!("{} ", item.title), Style::default().fg(Color::Gray)),
                    ])
                };
                ListItem::new(content)
            })
            .collect()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .border_style(Style::default().fg(Color::Rgb(80, 80, 80)))
                .borders(Borders::RIGHT)
                .title(title),
        )
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(60, 60, 60))
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(list, area);
}
