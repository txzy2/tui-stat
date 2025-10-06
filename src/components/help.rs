use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::components;

pub fn render_help_modal(frame: &mut Frame) {
    let modal_area = components::center_rect(50, 40, frame.area());

    let modal_block = Block::default()
        .title(
            Line::from(vec![
                Span::raw(" "),
                Span::styled(
                    "Help menu",
                    Style::default().fg(Color::Gray).bg(Color::Rgb(30, 30, 40)),
                ),
                Span::raw(" "),
            ])
            .alignment(Alignment::Center),
        )
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(Color::Gray))
        .style(Style::default().bg(Color::Rgb(30, 30, 40)));

    frame.render_widget(modal_block, modal_area);

    let content_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(1), // Title spacing
            Constraint::Min(1),    // Content
            Constraint::Length(1), // Controls
        ])
        .split(modal_area);

    let help_content = vec![
        Line::from(vec![
            Span::styled(
                "Navigation: ",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::raw(" "),
            Span::styled(
                "k/j ",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::styled(
                "(Up/Down)",
                Style::default()
                    .fg(Color::DarkGray)
                    .bg(Color::Rgb(30, 30, 40)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Actions:    ",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::raw(" "),
            Span::styled(
                "A ",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::styled(
                "(Add Task) ",
                Style::default()
                    .fg(Color::DarkGray)
                    .bg(Color::Rgb(30, 30, 40)),
            ),
            Span::styled(
                "D ",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::styled(
                "(Delete Task)",
                Style::default()
                    .fg(Color::DarkGray)
                    .bg(Color::Rgb(30, 30, 40)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Editing:    ",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::raw(" "),
            Span::styled(
                "T ",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::styled(
                "(Toggle Status)",
                Style::default()
                    .fg(Color::DarkGray)
                    .bg(Color::Rgb(30, 30, 40)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Input Modal:",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::raw(" "),
            Span::styled(
                "Enter ",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::styled(
                "(Save) ",
                Style::default()
                    .fg(Color::DarkGray)
                    .bg(Color::Rgb(30, 30, 40)),
            ),
            Span::styled(
                "Tab ",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::styled(
                "(Switch Field) ",
                Style::default()
                    .fg(Color::DarkGray)
                    .bg(Color::Rgb(30, 30, 40)),
            ),
            Span::styled(
                "Esc ",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::styled(
                "(Cancel)",
                Style::default()
                    .fg(Color::DarkGray)
                    .bg(Color::Rgb(30, 30, 40)),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "System:     ",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::raw(" "),
            Span::styled(
                "Q/Esc/? ",
                Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
            ),
            Span::styled(
                "(Quit/Help)",
                Style::default()
                    .fg(Color::DarkGray)
                    .bg(Color::Rgb(30, 30, 40)),
            ),
        ]),
    ];

    let help_text = Paragraph::new(help_content)
        .style(Style::default().bg(Color::Rgb(30, 30, 40)))
        .wrap(ratatui::widgets::Wrap { trim: true });

    let controls = Paragraph::new(
        Line::from("ESC (Close Help)".to_string()).alignment(ratatui::layout::Alignment::Center),
    )
    .style(Style::default().bg(Color::Rgb(30, 30, 40)));

    frame.render_widget(help_text, content_layout[1]);
    frame.render_widget(controls, content_layout[2]);
}
