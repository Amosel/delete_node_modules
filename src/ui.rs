use crate::{
    app::{App, GroupSelection},
    list::Filterable,
};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::*,
    Frame,
};

fn format_size(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;

    if bytes >= GIB {
        format!("{:.2} GB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.2} MB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.2} KB", bytes as f64 / KIB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(6), // Fixed size for the header
            Constraint::Min(0),    // Takes up the rest of the space
            Constraint::Length(3), // For status/feedback
        ])
        .split(frame.size());
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    frame.render_widget(
        Paragraph::new("\
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
            Press `up` and `down` to navigate and `space` to toggle selection\n\
            Press `a` or `Tab` to toggle selection between all, none or per item\n\
            Press `Enter` to delete currently selected items\n\
        ".to_string())
        .block(
            Block::default()
                .title("node_modules Cleaner")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        chunks[0],
    );

    if app.list.visible_items().next().is_some() {
        let mut selected_count: usize = 0;
        let mut selection_size: u64 = 0;
        let has_search_input = match app.filter_input.as_deref() {
            None => false,
            Some(s) => s.is_empty(),
        };

        let items: Vec<ListItem> = app
            .list
            .visible_items()
            .filter(|item| {
                if let Some(filter_input) = app.filter_input.as_ref() {
                    item.entry.path().to_str().unwrap().contains(filter_input)
                } else {
                    true
                }
            })
            // .filter(|item| item.entry.path().contains(&filter_input))
            .map(|item| {
                let mut is_on = false;

                if has_search_input {
                    is_on = true;
                    selected_count += 1;
                    selection_size += item.size;
                } else if let Some(group_selection) = &app.group_selection {
                    match group_selection {
                        GroupSelection::All => {}
                        GroupSelection::None => {
                            is_on = true;
                            selected_count += 1;
                            selection_size += item.size;
                        }
                    }
                } else if item.is_on {
                    is_on = true;
                    selected_count += 1;
                    selection_size += item.size;
                }

                let title: String = item.entry.path().to_str().unwrap().to_string()
                    + " - "
                    + &format_size(item.size);

                let select_char: &str = if item.deleting {
                    "[x] "
                } else if is_on {
                    "[•] "
                } else {
                    "[ ] "
                };
                ListItem::new(Line::from(vec![Span::raw(select_char), Span::raw(title)]))
                    .style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let middle_text = if app.scanning {
            ", Scanning...".to_string()
        } else if let Some(active) = app.deletes.active() {
            format!("Deleting {} ({})", active.0, format_size(active.1))
        } else {
            "".to_string()
        };
        let selected_number_text = if selected_count > 0 {
            format!("{}", selected_count)
        } else {
            "0".to_string()
        };
        let selection_size_text = format_size(selection_size);
        let search_text = if app.search {
            format!(
                " [ Search (toggle with /) {}]",
                app.filter_input.as_ref().unwrap_or(&"".to_string())
            )
        } else {
            "".to_string()
        };
        let title = format!(
            "Directories {}/{} {} Volume :{} --{}",
            selected_number_text,
            items.len(),
            middle_text,
            selection_size_text,
            search_text
        );
        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(title),
            )
            .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
            .highlight_symbol(">> ");

        // We can now render the item list
        frame.render_stateful_widget(list, chunks[1], &mut app.list.state);

        if app.search {
            if let Some(filter_input) = app.filter_input.as_ref() {
                // Assuming 'filter_input' holds the text entered by the user
                let filter_text = Paragraph::new(filter_input.clone())
                    .block(Block::default().borders(Borders::ALL).title("Filter"))
                    .style(
                        Style::default()
                            .bg(Color::Black)
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    );
                frame.render_widget(filter_text, chunks[2]);
            } else {
                let filter_block = Block::default().borders(Borders::ALL).title("Filter");
                frame.render_widget(filter_block, chunks[2]);
            }
        }
    } else if !app.scanning {
        frame.render_widget(
            Paragraph::new(
                "\
                \n\
                \n\
                No Items\n\
                ",
            )
            .block(
                Block::default()
                    .title("Empty")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Cyan).bg(Color::Black))
            .alignment(Alignment::Center),
            chunks[1],
        );
    }
}
