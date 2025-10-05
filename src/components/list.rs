use chrono::Local;
use ratatui::{
    prelude::Stylize,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::types::{ListState, TODOData};

impl Default for ListState {
    fn default() -> Self {
        Self {
            selected: Some(0),
            items: generate_random_list(),
        }
    }
}

//TODO: убрать после подключения локальной бaзы
fn generate_random_list() -> Vec<TODOData> {
    use crate::types::Status::*;
    let titles = [
        "Купить продукты",
        "Сдать проект",
        "Встреча с клиентом",
        "Техническое обслуживание",
        "Оплатить аренду",
    ];

    let messages = [
        "Купить продукты домой",
        "Сдать проект по разработке Рассчета коммисии",
        "Встреча с клиентом по ошибке",
        "Техническое обслуживание сервера тестирования",
        "Оплатить аренду за кв + вода",
    ];

    let statuses = [Active, Todo, Done, Cancelled, Active, Todo];

    let mut list: Vec<TODOData> = Vec::new();
    let mut i = 0;

    while i <= 5 {
        let todo = TODOData {
            id: i,
            title: titles[i as usize % titles.len()],
            message: messages[i as usize % messages.len()],
            date: Local::now(), // Assign the DateTime<Local> directly
            status: statuses[i as usize % statuses.len()],
        };
        list.push(todo);
        i += 1;
    }

    list
}

impl ListState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next(&mut self) {
        let len = self.items.len();
        if len == 0 {
            return;
        }

        let i = match self.selected {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    pub fn previous(&mut self) {
        let len = self.items.len();
        if len == 0 {
            return;
        }

        let i = match self.selected {
            Some(i) => {
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    pub fn selected_item(&self) -> Option<&TODOData> {
        self.selected.and_then(|i| self.items.get(i))
    }
}

pub fn render_list(frame: &mut Frame, area: ratatui::layout::Rect, state: &ListState) {
    let title = Line::from(vec![
        Span::raw("[== TODOS PANEL ==]"),
        Span::styled(" Move: ", Style::default().fg(Color::DarkGray)),
        Span::styled("k/j (Up/Down)", Style::default().fg(Color::White)),
    ])
    .bold()
    .centered();

    // Создаем список элементов для отображения
    let items: Vec<ListItem> = state
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
        .collect();

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
