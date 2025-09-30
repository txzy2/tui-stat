use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::{app::App, logger};
use color_eyre::eyre::Result;

pub struct KeyHandler;

impl KeyHandler {
    pub fn handle_crossterm_events(app: &mut App) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                Self::on_key_event(app, key);
            }
        }
        Ok(())
    }

    fn on_key_event(app: &mut App, key: KeyEvent) {
        if app.show_quit_modal {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => Self::quit(app),
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    app.show_quit_modal = false
                }
                _ => {}
            }
        } else {
            match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                    app.show_quit_modal = true;
                }
                // Обработка клавиш для перемещения по списку
                (_, KeyCode::Char('k')) => {
                    app.list_state.previous();
                }
                (_, KeyCode::Char('j')) => {
                    app.list_state.next();
                }
                (_, KeyCode::Enter) => {
                    app.show_item = !app.show_item;
                }
                _ => {}
            }
        }
    }

    fn quit(app: &mut App) {
        let _ = logger::info("Quit requested by user");
        app.request_quit();
    }
}
