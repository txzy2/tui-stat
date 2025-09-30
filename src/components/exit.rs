use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::components;

pub fn render_quit_modal(frame: &mut Frame) {
    let modal_area = components::center_rect(30, 12, frame.area());

    frame.render_widget(Clear, modal_area);

    let modal_block = Block::default()
        .title(
            Line::from(vec![
                Span::raw(" "),
                Span::styled(
                    "⚠ Выход из приложения",
                    Style::default()
                        .fg(Color::Gray)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
            ])
            .alignment(Alignment::Center),
        )
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Rgb(30, 30, 40)));

    frame.render_widget(modal_block, modal_area);

    let content_area = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(1), Constraint::Length(1)])
        .split(modal_area);

    let question = Paragraph::new(vec![Line::from(vec![
        Span::styled(
            "Вы уверены, что хотите выйти?",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " Y/N",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::ITALIC),
        ),
    ])
    .alignment(Alignment::Center)])
    .style(Style::default().bg(Color::Rgb(30, 30, 40)));

    let controls = Paragraph::new(Line::from("ESC (Отмена)").alignment(Alignment::Center))
        .style(Style::default().bg(Color::Rgb(30, 30, 40)));

    frame.render_widget(question, content_area[0]);
    frame.render_widget(controls, content_area[1]);
}
