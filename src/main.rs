use delete_node_modules::app::AppResult;
use delete_node_modules::dummy::dummy_app;
use delete_node_modules::event::{Event, EventHandler};
use delete_node_modules::handler::handle_key_events;
use delete_node_modules::item::Item;
use delete_node_modules::tui::Tui;
use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = dummy_app();

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
            Event::DirEntry(p) => app.push(Item::from_path(p).unwrap()),
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
