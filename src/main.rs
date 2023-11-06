use delete_node_modules::app::{App, AppResult};
use delete_node_modules::effects::walk_node_modules;
use delete_node_modules::event_handling::{handle_event, lifecycle};
use delete_node_modules::tui::Tui;
use std::io;
use std::sync::mpsc::channel;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::Terminal;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::default();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let (ui_sender, receiver) = channel();
    let handlers = vec![
        walk_node_modules(ui_sender.clone()),
        lifecycle(ui_sender.clone(), Duration::from_millis(250)),
    ];
    let mut tui = Tui::new(terminal, ui_sender, receiver, handlers);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        handle_event(tui.next()?, &mut app, &tui);
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
