mod exit;
mod help;
pub mod input;
pub mod list;
mod ram;
mod selected;
mod welcome;

pub use ram::format_sys_text;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(6),
            Constraint::Length(0),
        ])
        .split(frame.area());

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(3),
            Constraint::Length(0),
        ])
        .split(layout[1]);

    let main_chunks_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(40), Constraint::Min(40)])
        .split(main_chunks[0]);

    welcome::render_welcome(frame, layout[0], app);
    list::render_list(frame, main_chunks_split[0], &app.list_state);
    selected::render_select(frame, main_chunks_split[1], &app.list_state);
    ram::render_memory_info(frame, main_chunks[1], &app.sys_text);

    if app.show_quit_modal {
        exit::render_quit_modal(frame);
    }

    if app.show_help {
        help::render_help_modal(frame);
    }

    if app.show_add_modal {
        input::render_input_modal(frame, app);
    }
}

pub fn center_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_rect_dimensions() {
        let container = Rect::new(0, 0, 100, 100);
        let centered = center_rect(50, 50, container);

        assert!(centered.x > 0);
        assert!(centered.y > 0);
        assert!(centered.width <= 50);
        assert!(centered.height <= 50);
    }

    #[test]
    fn test_center_rect_small_percentage() {
        let container = Rect::new(0, 0, 200, 200);
        let centered = center_rect(20, 20, container);

        assert!(centered.width < container.width / 2);
        assert!(centered.height < container.height / 2);
    }
}
