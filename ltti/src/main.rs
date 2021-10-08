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
use gui::games::tic_tac_toe;
use gui::tick::{background_terminal_events_and_ticks, Event::*};
use gui::GameUserInterface;

use lttcore::Play;

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
        terminal.draw(|frame| {
            let chunks = layout().split(frame.size());

            let game: ::tic_tac_toe::TicTacToe =
                ::tic_tac_toe::Board::from_ints([[0, 1, 2], [1, 0, 1], [2, 2, 0]]).into();

            use lttcore::player::p;
            let settings = ::tic_tac_toe::Settings::new([p(1), p(2)]);

            ::tic_tac_toe::TicTacToe::render_action_request(
                frame,
                chunks[1],
                &Default::default(),
                &settings,
                &game.player_view(),
                &game.spectator_view(),
                &game.action_requests(&settings)[0].1,
                |x| {
                    println!("{:?}", x);
                },
            );

            frame.render_widget(footer(), chunks[2]);

            match events_reciever.recv().expect("foobar") {
                Tick => {}
                Resize => {}
                Input(_event) => {}
            }
        })?;
    }
}
