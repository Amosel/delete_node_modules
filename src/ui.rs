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

fn format_number(n: u64) -> String {
    let mut result = n.to_string();
    let mut pos = result.len();
    while pos > 3 {
        pos -= 3;
        result.insert(pos, ',');
    }
    result
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
        Paragraph::new(
            "\
            Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
            Press `up` and `down` to navigate and `space` to toggle selection\n\
            Press `a` or `Tab` to toggle selection between all, none or per item\n\
            Press `Enter` to delete currently selected items\n\
        "
            .to_string(),
        )
        .block(
            Block::default()
                .title(" node_modules Cleaner ")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        chunks[0],
    );

    if app.list.has_visible_items() {
        let filter_input = app.filter_input.as_ref();
        let has_search_input = filter_input.map(|s| !s.is_empty()).unwrap_or(false);

        let items: Vec<ListItem> = app
            .list
            .visible_items()
            .filter(|item| {
                if let Some(filter_input) = filter_input {
                    item.entry.path().to_str().unwrap().contains(filter_input)
                } else {
                    true
                }
            })
            // .filter(|item| item.entry.path().contains(&filter_input))
            .map(|item: &crate::dir_entry_item::DirEntryItem| {
                let mut is_on = false;
                if has_search_input {
                    is_on = false;
                } else if let Some(GroupSelection::None) = app.list.group_selection {
                    is_on = false;
                } else if item.is_on() {
                    is_on = true;
                }

                let title = format!(
                    "{} - {}",
                    item.entry.path().display(),
                    format_size(item.size)
                );

                let select_char = if item.is_deleting() {
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
        let middle_text = if app.list.is_scanning() {
            ", Scanning...".to_string()
        } else if app.deleting_size.current.count > 0 {
            format!(
                "Deleting {} ({})",
                app.deleting_size.current.count,
                format_size(app.deleting_size.current.total_size)
            )
        } else {
            "".to_string()
        };
        let selected_number_text = format!("{}", app.selected.count);
        let selection_size_text = format_size(app.selected.total_size);
        let search_text = if app.is_in_search_mode {
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

        if app.is_in_search_mode {
            if let Some(filter_input) = app.filter_input.as_ref() {
                // Assuming 'filter_input' holds the text entered by the user
                let filter_text = Paragraph::new(filter_input.clone())
                    .block(Block::default().borders(Borders::ALL).title("Filter "))
                    .style(
                        Style::default()
                            .bg(Color::Black)
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    );
                frame.render_widget(filter_text, chunks[2]);
            } else {
                let filter_block = Block::default().borders(Borders::ALL).title(" Filter");
                frame.render_widget(filter_block, chunks[2]);
            }
        }
    } else {
        let (title, text) = if app.list.done_scanning() {
            (
                " Empty ".to_string(),
                format!(
                    "\
                \n\
                \n\
                No Items found\n\
                {} Files scanned\n\
                ",
                    format_number(app.search_counter),
                ),
            )
        } else {
            (
                format!(" Scanning ({}) ", format_number(app.search_counter)),
                "\
                \n\
                \n\
                No Items found Yet\n\
                "
                .to_string(),
            )
        };

        frame.render_widget(
            Paragraph::new(text)
                .block(
                    Block::default()
                        .title(title)
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
