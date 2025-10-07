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
        if app.show_add_modal {
            // Handle input modal events
            match key.code {
                KeyCode::Enter => {
                    // Save the new TODO if title is not empty
                    if !app.input_title.is_empty() {
                        if let Err(e) = app.add_todo_to_db(
                            &app.input_title,
                            &app.input_message,
                            crate::types::Status::Todo,
                        ) {
                            let _ = logger::error(format!("Error adding TODO to DB: {}", e));
                        } else if let Err(e) = app.load_todos_from_db() {
                            let _ = logger::error(format!(
                                "Error loading todos from DB after add: {}",
                                e
                            ));
                        }
                    }
                    // Exit input mode
                    app.show_add_modal = false;
                    app.input_title.clear();
                    app.input_message.clear();
                    app.input_current_field = crate::app::InputField::Title;
                }
                KeyCode::Esc => {
                    // Cancel input
                    app.show_add_modal = false;
                    app.input_title.clear();
                    app.input_message.clear();
                    app.input_current_field = crate::app::InputField::Title;
                }
                KeyCode::Tab => {
                    // Switch between input fields
                    app.input_current_field = match app.input_current_field {
                        crate::app::InputField::Title => {
                            // Update cursor position for the new field
                            app.input_cursor_pos = app.input_message.len();
                            crate::app::InputField::Message
                        }
                        crate::app::InputField::Message => {
                            // Update cursor position for the new field
                            app.input_cursor_pos = app.input_title.len();
                            crate::app::InputField::Title
                        }
                    };
                }
                KeyCode::Backspace => {
                    // Handle backspace in the current input field
                    match app.input_current_field {
                        crate::app::InputField::Title => {
                            if !app.input_title.is_empty() {
                                app.input_title.pop();
                                // Update cursor position
                                app.input_cursor_pos = app.input_title.len();
                            }
                        }
                        crate::app::InputField::Message => {
                            if !app.input_message.is_empty() {
                                app.input_message.pop();
                                // Update cursor position
                                app.input_cursor_pos = app.input_message.len();
                            }
                        }
                    }
                }
                KeyCode::Char(c) => match app.input_current_field {
                    crate::app::InputField::Title => {
                        if app.input_title.len() < 50 {
                            app.input_title.push(c);
                            app.input_cursor_pos = app.input_title.len();
                        }
                    }
                    crate::app::InputField::Message => {
                        if app.input_message.len() < 200 {
                            app.input_message.push(c);
                            app.input_cursor_pos = app.input_message.len();
                        }
                    }
                },
                _ => {}
            }
        } else if app.show_quit_modal {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => Self::quit(app),
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    app.show_quit_modal = false
                }
                _ => {}
            }
        } else if app.show_help {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => app.show_help = false,
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
                (_, KeyCode::Char('?')) => app.show_help = true,
                (_, KeyCode::Char('T')) => {
                    if let Some(i) = app.list_state.selected {
                        let current_status = app.list_state.items[i].status;
                        let new_status = match current_status {
                            crate::types::Status::Todo => crate::types::Status::Active,
                            crate::types::Status::Active => crate::types::Status::Done,
                            crate::types::Status::Done => crate::types::Status::Cancelled,
                            crate::types::Status::Cancelled => crate::types::Status::Todo,
                        };

                        if let Err(e) =
                            app.update_todo_status_in_db(app.list_state.items[i].id, new_status)
                        {
                            let _ =
                                logger::error(format!("Error updating TODO status in DB: {}", e));
                        } else if let Err(e) = app.load_todos_from_db() {
                            let _ = logger::error(format!(
                                "Error loading todos from DB after status update: {}",
                                e
                            ));
                        }
                    }
                }
                (_, KeyCode::Char('A')) => {
                    // Enter input mode for adding a new TODO
                    app.show_add_modal = true;
                    app.input_title.clear();
                    app.input_message.clear();
                    app.input_cursor_pos = 0;
                    app.input_current_field = crate::app::InputField::Title;
                }
                (_, KeyCode::Char('D')) => {
                    if let Some(i) = app.list_state.selected {
                        if let Err(e) = app.delete_todo_from_db(app.list_state.items[i].id) {
                            let _ = logger::error(format!("Error deleting TODO from DB: {}", e));
                        } else if let Err(e) = app.load_todos_from_db() {
                            let _ = logger::error(format!(
                                "Error loading todos from DB after delete: {}",
                                e
                            ));
                        }
                    }
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
