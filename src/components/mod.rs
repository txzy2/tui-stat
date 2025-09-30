mod footer;
mod ram;
mod welcome;

use ratatui::prelude::Stylize;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::logger;
struct GenChunksParams {
    areas: Vec<Rect>,
    constraints: Vec<u16>,
    direction: Direction,
}

fn generate_chunks(params: GenChunksParams) -> Result<Vec<Rect>, String> {
    let GenChunksParams {
        areas,
        constraints,
        direction,
    } = params;

    if constraints.iter().sum::<u16>() != 100 {
        return Err("Constraints must sum to 100".to_string());
    }

    let constraints: Vec<Constraint> = constraints
        .into_iter()
        .map(Constraint::Percentage)
        .collect();

    let mut chunks = Vec::new();

    for area in areas {
        let splits = Layout::default()
            .direction(direction)
            .constraints(constraints.clone())
            .split(area);

        chunks.extend(splits.iter());
    }

    Ok(chunks)
}

pub fn render(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(99), Constraint::Percentage(1)])
        .split(frame.area());

    let main_chunks = match generate_chunks(GenChunksParams {
        areas: layout.to_vec(),
        constraints: vec![50, 50],
        direction: Direction::Horizontal,
    }) {
        Ok(chunks) => chunks,
        Err(e) => {
            let _ = logger::error(format!("Failed to generate main chunks: {}", e));
            return;
        }
    };

    let left_chunks = match generate_chunks(GenChunksParams {
        areas: main_chunks.to_vec(),
        constraints: vec![97, 3],
        direction: Direction::Vertical,
    }) {
        Ok(chunks) => chunks,
        Err(e) => {
            let _ = logger::error(format!("Failed to generate left chunks: {}", e));
            return;
        }
    };

    // let right_chunks = generate_chunks(&main_chunks, 20, 20);

    welcome::render_welcome(frame, left_chunks[0], app);
    ram::render_memory_info(frame, left_chunks[1], &app.ram_text);
    footer::render_footer(frame, layout[1]);

    if app.show_quit_modal {
        render_quit_modal(frame);
    }
}

fn render_quit_modal(frame: &mut Frame) {
    // Создаем затемнение фона
    let overlay = Block::default().style(Style::default().bg(Color::DarkGray));
    frame.render_widget(overlay, frame.area());

    // Создаем модальное окно по центру
    let modal_area = center_rect(30, 10, frame.area());

    let modal_block = Block::default()
        .title(Line::from("Выход из приложения").red().centered())
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .style(Style::default().fg(Color::Black));

    frame.render_widget(modal_block, modal_area);

    // Содержимое модального окна
    let content = Paragraph::new(
        "Вы уверены, что хотите выйти? Y/N\n\
             Esc - Отмена",
    )
    .centered()
    .style(Style::default().fg(Color::White));

    let content_area = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Min(1)])
        .split(modal_area);

    frame.render_widget(content, content_area[0]);
}

/// Helper function to center a rectangle
fn center_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
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
    fn test_generate_chunks_valid_constraints() {
        let test_rect = Rect::new(0, 0, 100, 100);

        let result = generate_chunks(GenChunksParams {
            areas: vec![test_rect],
            constraints: vec![50, 50],
            direction: Direction::Horizontal,
        });

        assert!(result.is_ok(), "Valid constraints should succeed");
        let chunks = result.unwrap();
        assert_eq!(
            chunks.len(),
            2,
            "Should generate 2 chunks for 2 constraints"
        );
    }

    #[test]
    fn test_generate_chunks_invalid_constraints() {
        let test_rect = Rect::new(0, 0, 100, 100);

        let result = generate_chunks(GenChunksParams {
            areas: vec![test_rect],
            constraints: vec![50, 30], // Сумма != 100
            direction: Direction::Horizontal,
        });

        assert!(result.is_err(), "Invalid constraints should fail");
        assert_eq!(result.unwrap_err(), "Constraints must sum to 100");
    }

    #[test]
    fn test_generate_chunks_sum_exactly_100() {
        let test_rect = Rect::new(0, 0, 100, 100);

        let result = generate_chunks(GenChunksParams {
            areas: vec![test_rect],
            constraints: vec![33, 33, 34], // Сумма = 100
            direction: Direction::Vertical,
        });

        assert!(result.is_ok(), "Constraints summing to 100 should succeed");
        let chunks = result.unwrap();
        assert_eq!(
            chunks.len(),
            3,
            "Should generate 3 chunks for 3 constraints"
        );
    }

    #[test]
    fn test_generate_chunks_multiple_areas() {
        let rect1 = Rect::new(0, 0, 50, 50);
        let rect2 = Rect::new(50, 50, 50, 50);

        let result = generate_chunks(GenChunksParams {
            areas: vec![rect1, rect2],
            constraints: vec![60, 40],
            direction: Direction::Horizontal,
        });

        assert!(result.is_ok());
        let chunks = result.unwrap();
        // 2 области * 2 ограничения = 4 chunks
        assert_eq!(chunks.len(), 4, "Should generate chunks for each area");
    }

    #[test]
    fn test_center_rect_dimensions() {
        let container = Rect::new(0, 0, 100, 100);

        let centered = center_rect(50, 50, container);

        // Центрированный rect должен быть примерно в центре
        assert!(centered.x > 0, "X should be offset from left");
        assert!(centered.y > 0, "Y should be offset from top");
        assert!(
            centered.width <= 50,
            "Width should be at most 50% of container"
        );
        assert!(
            centered.height <= 50,
            "Height should be at most 50% of container"
        );
    }

    #[test]
    fn test_center_rect_small_percentage() {
        let container = Rect::new(0, 0, 200, 200);

        let centered = center_rect(20, 20, container);

        // Маленький процент должен давать маленький rect с большими отступами
        assert!(centered.width < container.width / 2);
        assert!(centered.height < container.height / 2);
    }
}
