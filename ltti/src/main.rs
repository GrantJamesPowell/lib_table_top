#![allow(unused_imports)]
#![allow(dead_code)]

use tui::backend::CrosstermBackend;
use tui::{Frame, Terminal};

use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::symbols::DOT;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Tabs};

use std::{io, sync::mpsc, thread, time::Duration};

mod gui;

use gui::common::{footer, layout};
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
            let chunks = layout().split(rect.size());

            rect.render_widget(footer(), chunks[2]);

            match events_reciever.recv().expect("foobar") {
                Tick => {}
                Resize => {}
                Input(_event) => {}
            }
        })?;
    }
}