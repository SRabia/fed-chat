use crate::app::{App, AppResult, AppState};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use tui_input::backend::crossterm::EventHandler;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.state {
        AppState::Normal => match key_event.code {
            KeyCode::Char('q') => {
                app.quit();
            }
            KeyCode::Char('c') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.quit();
                }
            }
            KeyCode::Char('i') => {
                app.editing();
            }
            KeyCode::Char('j') | KeyCode::Down => {
                app.scroll_down();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.scroll_up();
            }
            _ => {}
        },
        AppState::Editing => match key_event.code {
            KeyCode::Enter => {
                app.messages.push(app.input.value().into());
                app.input.reset();
            }
            KeyCode::Char('c') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.normal();
                }
            }
            KeyCode::Esc => {
                app.normal();
            }
            _ => {
                app.input.handle_event(&Event::Key(key_event));
            }
        },
        _ => {}
    }

    Ok(())
}
