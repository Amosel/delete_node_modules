use crossterm::event::{KeyCode, KeyModifiers};
use delete_node_modules::app::{App, AppResult};
use delete_node_modules::event::{delete, Event, EventHandler};
use delete_node_modules::tui::Tui;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::default();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => {
                if app.search {
                    match key_event.code {
                        KeyCode::Esc => {
                            app.end_search_entry();
                        }
                        KeyCode::Enter | KeyCode::Char('/') => {
                            app.end_search_entry();
                        }
                        KeyCode::Char(c) => {
                            // Append the character to your filter_input
                            app.append_filter_input(c);
                        }
                        KeyCode::Backspace => {
                            // Remove the last character from filter_input
                            app.delete_filter_input();
                        }
                        // ... handle other keys ...
                        _ => {}
                    }
                } else {
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
                            app.toggle_selected_item();
                        }
                        KeyCode::Right => app.set_on_and_next(),
                        KeyCode::Left => app.set_off_and_next(),
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Tab => {
                            app.toggle_group_selection();
                        }
                        KeyCode::Char('a') | KeyCode::Char('A') => {
                            app.toggle_group_selection();
                        }
                        KeyCode::Char('/') => {
                            app.start_search_entry();
                        }
                        KeyCode::Enter => {
                            let items = app.items_to_delete();
                            if !items.is_empty() {
                                delete(app.items_to_delete(), &tui.events.sender);
                            }
                        }
                        // Other handlers you could add here.
                        _ => {}
                    }
                }
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::Entry(e) => app.handle_entry(e),
            Event::Delete(e) => app.handle_delete(e),
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
