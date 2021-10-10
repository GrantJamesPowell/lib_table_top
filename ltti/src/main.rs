#![allow(unused_imports)]
#![allow(dead_code)]

use tui::backend::CrosstermBackend;
use tui::{Frame, Terminal};

use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::symbols::DOT;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Tabs};

use std::{error::Error, io, sync::mpsc, thread, time::Duration};

mod gui;

use gui::common::layout;
use gui::games::tic_tac_toe;
use gui::tick::{background_terminal_events_and_ticks, Event::*};

use gui::game_ui::action_request::{
    ActionRequestContext, ActionRequestInterface, ActionRequestState,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use lttcore::Play;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (events_sender, events_reciever) = mpsc::channel();
    let tick_rate = Duration::from_millis(100);

    thread::spawn(background_terminal_events_and_ticks(
        tick_rate,
        events_sender,
    ));

    let game: ::tic_tac_toe::TicTacToe =
        ::tic_tac_toe::Board::from_ints([[1, 0, 2], [0, 0, 0], [2, 2, 0]]).into();

    use lttcore::player::p;
    let settings = ::tic_tac_toe::Settings::new([p(1), p(2)]);

    let mut ui_state = Default::default();
    let action_requests = game.action_requests(&settings);

    let context: ActionRequestContext<::tic_tac_toe::TicTacToe> = ActionRequestContext {
        player: action_requests[0].0,
        action_request: action_requests[0].1,
        player_view: game.player_view(),
        spectator_view: game.spectator_view(),
    };

    loop {
        terminal.draw(|frame| {
            let chunks = layout().split(frame.size());

            ::tic_tac_toe::TicTacToe::render_action_request(
                frame, chunks[1], &context, &settings, &ui_state,
            );
        })?;

        match events_reciever.recv().expect("foobar") {
            Tick => {}
            Resize => {}
            Input(event) => match event.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    disable_raw_mode()?;
                    execute!(
                        terminal.backend_mut(),
                        LeaveAlternateScreen,
                        DisableMouseCapture
                    )?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => ui_state.on_input(event, &context, &settings, |action| {
                    println!("{:?}", action);
                }),
            },
        };
    }

    Ok(())
}
