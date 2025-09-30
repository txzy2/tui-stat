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
    let title = Line::from("TODOS").bold().white().centered();

    // Создаем список элементов для отображения
    let items: Vec<ListItem> = state
        .items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let content = if state.selected == Some(i) {
                Line::from(vec![
                    Span::styled(
                        format!("   > [{:?}]", item.status),
                        Style::default().fg(item.status.get_color()),
                    ),
                    Span::styled(
                        format!(" {}: {} ", item.id, item.title), // Added 3 spaces for padding
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::styled(
                        format!("   {}: {} ", item.id, item.title), // Added 3 spaces for padding
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!("[{:?}]", item.status),
                        Style::default().fg(Color::White),
                    ),
                ])
            };
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .border_style(Style::default().fg(Color::White))
                .borders(Borders::RIGHT)
                .title(title),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    frame.render_widget(list, area);
}
