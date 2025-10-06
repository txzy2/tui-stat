use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::{App, InputField};
use crate::components::center_rect;

pub fn render_input_modal(frame: &mut Frame, app: &App) {
    let modal_area = center_rect(50, 30, frame.area());
    frame.render_widget(Clear, modal_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray))
        .border_type(BorderType::Rounded)
        .title("Add New TODO");

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title
            Constraint::Length(3), // Title input
            Constraint::Length(3), // Message input
            Constraint::Length(1), // Instructions
            Constraint::Fill(1),   // Empty space
        ])
        .split(modal_area);

    let title_border_style = if app.input_current_field == InputField::Title {
        Style::default().fg(Color::White) // Active field
    } else {
        Style::default().fg(Color::DarkGray) // Inactive field
    };

    let message_border_style = if app.input_current_field == InputField::Message {
        Style::default().fg(Color::White) // Active field
    } else {
        Style::default().fg(Color::DarkGray) // Inactive field
    };

    let title_input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(title_border_style)
        .title("Title");

    let title_text = if app.input_title.is_empty() {
        Span::raw("Enter title...")
    } else {
        Span::raw(&app.input_title)
    };

    let title_paragraph = Paragraph::new(title_text)
        .block(title_input_block)
        .style(Style::default().fg(Color::White));

    frame.render_widget(title_paragraph, vertical_layout[1]);

    let message_input_block = Block::default()
        .borders(Borders::ALL)
        .border_style(message_border_style)
        .title("Message");

    let message_text = if app.input_message.is_empty() {
        Span::raw("Enter description...")
    } else {
        Span::raw(&app.input_message)
    };

    let message_paragraph = Paragraph::new(message_text)
        .block(message_input_block)
        .style(Style::default().fg(Color::White));

    frame.render_widget(message_paragraph, vertical_layout[2]);

    let instructions = Line::from(vec![Span::raw(
        "  Enter - save, Tab - switch field, Esc - cancel",
    )]);

    let instructions_paragraph =
        Paragraph::new(instructions).style(Style::default().fg(Color::DarkGray));

    frame.render_widget(instructions_paragraph, vertical_layout[3]);
    frame.render_widget(block, modal_area);
}
