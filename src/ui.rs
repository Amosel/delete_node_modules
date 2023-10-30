use crate::app::{App, GroupSelection};
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
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.size());

    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples
    frame.render_widget(
        Paragraph::new(format!(
            "\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press up and down to navigate and space bad to toggle selection\n\
                Press 'a' to toggle selecting all, deselecting all or per item selection\n\
                ",
        ))
        .block(
            Block::default()
                .title("node_modules Cleaner")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan).bg(Color::Black))
        .alignment(Alignment::Center),
        layout[0],
    );

    if !app.list.items.is_empty() {
        let mut selected_count: usize = 0;
        let mut selection_size: u64 = 0;
        let items: Vec<ListItem> = app
            .list
            .items
            .iter()
            .map(|item| {
                let mut select_char: &str = "[ ] ";
                match app.group_selection {
                    GroupSelection::Deselected => {}
                    GroupSelection::Selected => {
                        select_char = "[•] ";
                        selected_count += 1;
                        selection_size += item.size;
                    }
                    GroupSelection::None => {
                        if !item.is_on {
                            select_char = "[ ] ";
                        } else {
                            select_char = "[•] ";
                            selected_count += 1;
                            selection_size += item.size;
                        }
                    }
                };

                let title: String = item
                    .entry
                    .path()
                    .to_str()
                    .unwrap()
                    .to_string()
                    // .replace("node_modules", "")
                    + " - "
                    + &format_size(item.size);
                ListItem::new(Line::from(vec![Span::raw(select_char), Span::raw(title)]))
                    .style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let is_loading_text = if app.loading { ", Loading..." } else { "" };
        let selected_number_text = if selected_count > 0 {
            format!("{}", selected_count)
        } else {
            "0".to_string()
        };
        let selection_size_text = format_size(selection_size);
        let title = format!(
            "Directories {}/{} {} selected, {}",
            selected_number_text,
            app.list.items.len(),
            is_loading_text,
            selection_size_text
        );
        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(title),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Black)
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        // We can now render the item list
        frame.render_stateful_widget(list, layout[1], &mut app.list.state);
    } else if !app.loading {
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
            layout[1],
        );
    }
}
