use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::components;

pub fn render_help_modal(frame: &mut Frame) {
    let modal_area = components::center_rect(40, 50, frame.area());

    // Clear the area to avoid overlapping
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::default()
        .title(
            Line::from(vec![
                Span::styled(
                    " HELP ",
                    Style::default()
                        .fg(Color::LightBlue)
                        .bg(Color::Rgb(30, 30, 40))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(" "),
            ])
            .alignment(Alignment::Center),
        )
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .border_style(Style::default().fg(Color::DarkGray))
        .style(Style::default().bg(Color::Rgb(25, 25, 35)));

    frame.render_widget(modal_block, modal_area);

    let content_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(2)
        .vertical_margin(1)
        .constraints([
            Constraint::Length(2), // Header
            Constraint::Min(1),    // Content
            Constraint::Length(2), // Footer
        ])
        .split(modal_area);

    // Define color styles
    let category_style = Style::default()
        .fg(Color::Cyan)
        .bg(Color::Rgb(25, 25, 35))
        .add_modifier(Modifier::BOLD);
    let key_style = Style::default()
        .fg(Color::Yellow)
        .bg(Color::Rgb(25, 25, 35))
        .add_modifier(Modifier::BOLD);
    let description_style = Style::default().fg(Color::Gray).bg(Color::Rgb(25, 25, 35));

    let help_content = vec![
        // Navigation section
        Line::from(vec![Span::styled(" NAVIGATION ", category_style)]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled("k", key_style),
            Span::styled(" - Move up in list", description_style),
        ]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled("j", key_style),
            Span::styled(" - Move down in list", description_style),
        ]),
        Line::from(""),
        // Task Actions section
        Line::from(vec![Span::styled(" TASK ACTIONS ", category_style)]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled("A", key_style),
            Span::styled(" - Add new task", description_style),
        ]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled("D", key_style),
            Span::styled(" - Delete selected task", description_style),
        ]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled("T", key_style),
            Span::styled(" - Toggle task status", description_style),
        ]),
        Line::from(""),
        // Input Modal section
        Line::from(vec![Span::styled(" INPUT MODAL ", category_style)]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled("Enter", key_style),
            Span::styled(" - Save task", description_style),
        ]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled("Tab", key_style),
            Span::styled(" - Switch between title/message fields", description_style),
        ]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled("Esc", key_style),
            Span::styled(" - Cancel and close modal", description_style),
        ]),
        Line::from(""),
        // System Controls section
        Line::from(vec![Span::styled(" SYSTEM CONTROLS ", category_style)]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled("Q / Ctrl+C ", key_style),
            Span::styled(" - Show quit confirmation", description_style),
        ]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled("?", key_style),
            Span::styled(" - Show this help menu", description_style),
        ]),
    ];

    let help_text = Paragraph::new(help_content)
        .style(Style::default().bg(Color::Rgb(25, 25, 35)))
        .alignment(Alignment::Left)
        .wrap(ratatui::widgets::Wrap { trim: true });

    let controls = Paragraph::new(
        Line::from(vec![
            Span::raw(" "),
            Span::styled(
                "ESC",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" to close help", Style::default().fg(Color::Gray)),
            Span::raw(" "),
        ])
        .alignment(Alignment::Center),
    )
    .style(Style::default().bg(Color::Rgb(25, 25, 35)));

    // Create a narrower centered area within the content area to effectively center the help content while keeping text left-aligned
    let centered_content_area = components::center_rect(70, 100, content_layout[1]);
    frame.render_widget(help_text, centered_content_area);
    frame.render_widget(controls, content_layout[2]);
}
