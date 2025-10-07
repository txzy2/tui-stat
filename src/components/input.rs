use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, InputField};
use crate::components::center_rect;

pub fn render_input_modal(frame: &mut Frame, app: &App) {
    let modal_area = center_rect(50, 30, frame.area());
    frame.render_widget(Clear, modal_area);

    let modal_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .border_type(BorderType::Double)
        .style(Style::default().bg(Color::Rgb(25, 25, 35)))
        .title(Line::from(vec![
            Span::styled(
                " ADD NEW TODO ",
                Style::default()
                    .fg(Color::Cyan)
                    .bg(Color::Rgb(25, 25, 35))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
        ]));

    frame.render_widget(modal_block, modal_area);

    let content_layout = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(2)
        .vertical_margin(1)
        .constraints([
            Constraint::Length(1), // Header separator
            Constraint::Length(1), // Field label
            Constraint::Length(3), // Title input
            Constraint::Length(1), // Field label
            Constraint::Length(5), // Message input (increased height)
            Constraint::Length(2), // Instructions
            Constraint::Min(1),    // Empty space
        ])
        .split(modal_area);

    // Define color styles
    let active_field_style = Style::default()
        .fg(Color::LightBlue)
        .add_modifier(Modifier::BOLD);
    let inactive_field_style = Style::default().fg(Color::DarkGray);
    let label_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
        .bg(Color::Rgb(25, 25, 35));

    // Title label
    let title_label = Paragraph::new(Line::from(vec![
        Span::styled("TITLE", label_style),
        Span::raw(" ".repeat(45)),
    ]))
    .style(Style::default().bg(Color::Rgb(25, 25, 35)));
    frame.render_widget(title_label, content_layout[1]);

    // Title input field
    let title_border_style = if app.input_current_field == InputField::Title {
        active_field_style
    } else {
        inactive_field_style
    };

    let title_input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(title_border_style)
        .style(Style::default().bg(Color::Rgb(30, 30, 40)));

    let title_text = if app.input_title.is_empty() {
        Span::styled(
            "Enter title... (max 50 chars)",
            Style::default()
                .fg(Color::DarkGray)
                .bg(Color::Rgb(30, 30, 40)),
        )
    } else {
        Span::styled(
            &app.input_title,
            Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
        )
    };

    let title_paragraph = Paragraph::new(title_text)
        .block(title_input_block)
        .style(Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)));

    frame.render_widget(title_paragraph, content_layout[2]);

    // Message label
    let message_label = Paragraph::new(Line::from(vec![
        Span::styled("MESSAGE", label_style),
        Span::raw(" ".repeat(43)),
    ]))
    .style(Style::default().bg(Color::Rgb(25, 25, 35)));
    frame.render_widget(message_label, content_layout[3]);

    // Message input field
    let message_border_style = if app.input_current_field == InputField::Message {
        active_field_style
    } else {
        inactive_field_style
    };

    let message_input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(message_border_style)
        .style(Style::default().bg(Color::Rgb(30, 30, 40)));

    let message_text = if app.input_message.is_empty() {
        Span::styled(
            "Enter description... (max 200 chars)",
            Style::default()
                .fg(Color::DarkGray)
                .bg(Color::Rgb(30, 30, 40)),
        )
    } else {
        Span::styled(
            &app.input_message,
            Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)),
        )
    };

    let message_paragraph = Paragraph::new(message_text)
        .block(message_input_block)
        .style(Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 40)))
        .wrap(Wrap { trim: true });

    frame.render_widget(message_paragraph, content_layout[4]);

    if app.input_current_field == InputField::Title {
        let start_x = content_layout[2].x + 1;
        let cursor_x = if app.input_title.is_empty() {
            start_x
        } else {
            let cursor_pos = std::cmp::min(app.input_cursor_pos, app.input_title.len());
            if cursor_pos > 0 {
                let visible_text = &app.input_title[..cursor_pos];
                start_x + visible_text.chars().count() as u16
            } else {
                start_x
            }
        };
        let cursor_y = content_layout[2].y + 1;
        frame.set_cursor_position((cursor_x, cursor_y));
    }

    if app.input_current_field == InputField::Message {
        let start_x = content_layout[4].x + 1;
        let cursor_x = if app.input_message.is_empty() {
            start_x
        } else {
            let cursor_pos = std::cmp::min(app.input_cursor_pos, app.input_message.len());
            if cursor_pos > 0 {
                let visible_text = &app.input_message[..cursor_pos];
                start_x + visible_text.chars().count() as u16
            } else {
                start_x
            }
        };
        let cursor_y = content_layout[4].y + 1;
        frame.set_cursor_position((cursor_x, cursor_y));
    }

    let instructions = Line::from(vec![
        Span::styled(
            " │ ",
            Style::default()
                .fg(Color::DarkGray)
                .bg(Color::Rgb(25, 25, 35)),
        ),
        Span::styled(
            " ENTER ",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Save ", Style::default().fg(Color::Gray)),
        Span::styled(
            " │ ",
            Style::default()
                .fg(Color::DarkGray)
                .bg(Color::Rgb(25, 25, 35)),
        ),
        Span::styled(
            " TAB ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Switch ", Style::default().fg(Color::Gray)),
        Span::styled(
            " │ ",
            Style::default()
                .fg(Color::DarkGray)
                .bg(Color::Rgb(25, 25, 35)),
        ),
        Span::styled(
            " ESC ",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Cancel ", Style::default().fg(Color::Gray)),
        Span::styled(
            "│ ",
            Style::default()
                .fg(Color::DarkGray)
                .bg(Color::Rgb(25, 25, 35)),
        ),
    ]);

    let instructions_paragraph = Paragraph::new(instructions)
        .style(Style::default().bg(Color::Rgb(25, 25, 35)))
        .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(instructions_paragraph, content_layout[5]);
}
