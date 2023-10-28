use delete_node_modules::app::{App, AppResult};
use delete_node_modules::dir_entry_item::DirEntryItem;
use delete_node_modules::event::{DirEvent, Event, EventHandler};
use delete_node_modules::key_event_handler::handle_key_events;
use delete_node_modules::tui::Tui;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

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
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            Event::Dir(d) => match d {
                DirEvent::Started => {
                    app.loading = true;
                }
                DirEvent::Finished => {
                    app.loading = false;
                }
                DirEvent::DirEntry(e) => {
                    app.push(DirEntryItem::from_entry(e)?);
                }
            },
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
