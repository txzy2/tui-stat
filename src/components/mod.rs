mod exit;
mod main_right;
mod ram;
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
            Constraint::Percentage(1),
            Constraint::Percentage(98),
            Constraint::Percentage(1),
        ])
        .split(frame.area());

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(layout[1]);

    let main_chunks_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[0]);

    welcome::render_welcome(frame, layout[0], app);
    main_right::render_main_right(frame, main_chunks_split[0]);
    ram::render_memory_info(frame, main_chunks[1], &app.sys_text);

    if app.show_quit_modal {
        exit::render_quit_modal(frame);
    }
}

/// Helper function to center a rectangle
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
