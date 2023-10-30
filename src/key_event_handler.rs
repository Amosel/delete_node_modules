use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        KeyCode::Char(' ') => {
            app.toggle();
        }
        KeyCode::Right => app.set_on_and_next(),
        KeyCode::Left => app.set_off_and_next(),
        KeyCode::Down => app.next(),
        KeyCode::Up => app.previous(),
        KeyCode::Tab => {
            app.toggle_group();
        }
        KeyCode::Char('a') => {
            app.toggle_group();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
