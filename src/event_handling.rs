use crate::app::App;
use crate::dir_entry_item::DirEntryItem;
use crate::effects::delete_items;
use crate::event::Event;
use crate::tui::Tui;
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyModifiers};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};
use tui::backend::Backend;

pub fn lifecycle(sender: Sender<Event>, tick_rate: Duration) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or(tick_rate);

            if event::poll(timeout).expect("no events available") {
                match event::read().expect("unable to read event") {
                    CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                    CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                    CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                    _ => unimplemented!(),
                }
                .expect("failed to send terminal event")
            }

            if last_tick.elapsed() >= tick_rate {
                sender.send(Event::Tick).expect("failed to send tick event");
                last_tick = Instant::now();
            }
        }
    })
}

pub fn handle_event<B: Backend>(event: Event, app: &mut App, tui: &Tui<B, Event>) {
    match event {
        Event::Tick => app.tick(),
        Event::Key(key_event) => {
            if app.is_in_search_mode {
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
                        let items: Vec<DirEntryItem> =
                            app.list.items_to_delete().cloned().collect();
                        if !items.is_empty() {
                            delete_items(items, &tui.sender);
                        }
                    }
                    // Other handlers you could add here.
                    _ => {}
                }
            }
        }
        Event::Mouse(_) => {}
        Event::Resize(_, _) => {}
        Event::Search(e) => app.handle_search(e),
        Event::Delete(e) => app.handle_delete(e),
    }
}
