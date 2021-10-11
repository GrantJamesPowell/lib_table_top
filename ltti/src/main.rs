#![allow(unused_imports)]
#![allow(dead_code)]

use tui::backend::Backend;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::symbols::DOT;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, List, ListItem, Tabs};
use tui::{Frame, Terminal};

use std::collections::HashMap;
use std::{error::Error, io, sync::mpsc, thread, time::Duration};

mod gui;

use gui::common::layout;
use gui::games::tic_tac_toe;
use gui::tick::{background_terminal_events_and_ticks, Event::*};

use gui::game_ui::action_request::{ActionRequestInterface, ActionRequestState};

use ::tic_tac_toe::{Settings, TicTacToe};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use lttcore::{play::ActionResponse, player::p, GameRunner, GameRunnerBuilder, Play, View};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;

    let (events_sender, events_receiver) = mpsc::channel();
    let tick_rate = Duration::from_millis(100);

    thread::spawn(background_terminal_events_and_ticks(
        tick_rate,
        events_sender,
    ));

    let mut game_runner: GameRunner<TicTacToe> = GameRunnerBuilder::default()
        .settings(Settings::new([p(1), p(2)]))
        .build()
        .unwrap();

    let mut ui_state = Default::default();
    let mut turn = game_runner.turn();
    let (action_id, (player, action_request)) = turn
        .as_ref()
        .map(|t| t.action_request())
        .flatten()
        .expect("New game has a first turn");
    let player_view = game_runner.game().player_view();
    let mut spectator_view = game_runner.game().spectator_view();

    let (action_sender, action_receiver) = mpsc::channel();

    loop {
        if let Ok((action_id, action)) = action_receiver.try_recv() {
            if let Some(mut current_turn) = Option::take(&mut turn) {
                current_turn.submit_action(action_id, ActionResponse::Response(action));

                if current_turn.is_ready_to_submit() {
                    let game_advance = game_runner.submit_turn_mut(current_turn)?;

                    for update in &game_advance.spectator_view_updates {
                        spectator_view.update(&update)?;
                    }

                    turn = game_runner.turn();
                }
            }
        }

        terminal.draw(|frame| {
            let chunks = layout().split(frame.size());
            TicTacToe::render_action_request(
                frame,
                chunks[1],
                &player,
                &player_view,
                &spectator_view,
                &action_request,
                game_runner.settings(),
                &ui_state,
            );
        })?;

        match events_receiver
            .recv()
            .expect("ticking background thread is alive")
        {
            Tick => {}
            Resize => {}
            Input(event) => match event.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    break;
                }
                _ => {
                    let sender = action_sender.clone();

                    ui_state.on_input(
                        event,
                        &player,
                        &player_view,
                        &spectator_view,
                        &action_request,
                        game_runner.settings(),
                        |action| sender.send((action_id, action)).unwrap(),
                    )
                }
            },
        };
    }

    clean_up_terminal(terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn clean_up_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
