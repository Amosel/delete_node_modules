use crate::app::{App, GroupSelection};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::*,
    Frame,
};

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
                Press a to toggle selecting all and deselecting all\n\
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
        let items: Vec<ListItem> = app
            .list
            .items
            .iter()
            .map(|i| {
                let select_char: &str = match app.group_selection {
                    GroupSelection::Deselected => "[ ] ",
                    GroupSelection::Selected => "[•] ",
                    GroupSelection::None => {
                        if i.is_on {
                            "[ ] "
                        } else {
                            "[•] "
                        }
                    }
                };

                ListItem::new(Line::from(vec![
                    Span::raw(select_char),
                    Span::raw(i.title().clone()),
                ]))
                .style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let is_loading_text = if app.loading { ", Loading..." } else { "" };
        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title(format!(
                        "Directories {}{}",
                        app.list.items.len(),
                        is_loading_text
                    )),
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
