#![allow(unused_imports)]

use tui::backend::CrosstermBackend;
use tui::{Frame, Terminal};

use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::symbols::DOT;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Tabs};

use std::{io, sync::mpsc, thread, time::Duration};

mod gui;

use gui::tick::{background_terminal_events_and_ticks, Event::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let (events_sender, events_reciever) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    thread::spawn(background_terminal_events_and_ticks(
        tick_rate,
        events_sender,
    ));

    loop {
        terminal.draw(|rect| {
            let size = rect.size();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let items = [
                ListItem::new("Item 1"),
                ListItem::new("Item 2"),
                ListItem::new("Item 3"),
            ];
            let list: List = List::new(items)
                .block(Block::default().title("List").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                .highlight_symbol(">>");

            let titles = ["Tab1", "Tab2", "Tab3", "Tab4"]
                .iter()
                .cloned()
                .map(Spans::from)
                .collect();
            let tabs = Tabs::new(titles)
                .block(Block::default().title("Tabs").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(DOT);

            rect.render_widget(tabs, chunks[0]);
            rect.render_widget(list, chunks[1]);

            match events_reciever.recv().expect("foobar") {
                Tick => {
                }
                Resize => {
                }
                Input(event) => {
                }
            }
        })?;
    }
}
