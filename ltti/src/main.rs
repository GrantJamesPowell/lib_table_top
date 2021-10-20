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

use ::tic_tac_toe::TicTacToe;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use lttcore::{
    play::{ActionResponse::*, NoCustomSettings},
    GameRunner, GameRunnerBuilder, Play, Player, View,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let mut terminal = setup_terminal()?;

    //let (events_sender, events_receiver) = mpsc::channel();
    //let tick_rate = Duration::from_millis(100);

    //thread::spawn(background_terminal_events_and_ticks(
    //    tick_rate,
    //    events_sender,
    //));

    //let mut game_runner: GameRunner<TicTacToe> = GameRunnerBuilder::default()
    //    .settings(NoCustomSettings)
    //    .build()
    //    .unwrap();

    //let mut player_views = game_runner.player_views();
    //let mut spectator_view = game_runner.spectator_view();

    //let mut turn = game_runner.turn();

    //let (action_sender, action_receiver) = mpsc::channel();

    //let mut ui_state = Default::default();
    //loop {
    //    if let Ok((player, action)) = action_receiver.try_recv() {
    //        if let Some(mut current_turn) = Option::take(&mut turn) {
    //            current_turn.add_action(player, action)?;

    //            if current_turn.is_ready_to_submit() {
    //                let (new_state, game_advance) = game_runner.submit_turn(current_turn).unwrap();
    //                spectator_view.update(&game_advance.spectator_update);

    //                for (player, player_update) in &game_advance.player_updates {
    //                    player_views
    //                        .get_mut(player)
    //                        .map(|view| view.update(player_update));
    //                }

    //                game_runner = new_state;
    //                turn = game_runner.turn();
    //            }
    //        }
    //    }

    //    let on_input: Box<dyn FnOnce(KeyEvent)> =
    //        if let Some(current_turn) = turn.as_ref() {
    //            let current_player = current_turn
    //                .pending_action_requests()
    //                .next()
    //                .expect("at least one player needs to do something");

    //            terminal.draw(|frame| {
    //                let chunks = layout().split(frame.size());
    //                TicTacToe::render_action_request(
    //                    frame,
    //                    chunks[1],
    //                    current_player,
    //                    &player_views.get(&current_player).unwrap(),
    //                    &spectator_view,
    //                    game_runner.settings(),
    //                    &ui_state,
    //                    );
    //            })?;

    //            Box::new(|event| {
    //                ui_state.on_input(
    //                    event,
    //                    current_player.clone(),
    //                    (&player_views).get(&current_player).unwrap(),
    //                    &spectator_view,
    //                    game_runner.settings(),
    //                    |action| action_sender.clone().send((current_player, Response(action))).unwrap(),
    //                )

    //            })
    //        } else {
    //            Box::new(|event| println!("{:?}", event))
    //        };

    //    match events_receiver
    //        .recv()
    //        .expect("ticking background thread is alive")
    //    {
    //        Tick => {}
    //        Resize => {}
    //        Input(event) => match event.code {
    //            KeyCode::Char('q') | KeyCode::Esc => {
    //                break;
    //            }
    //            _ => {
    //                on_input(event);
    //            }
    //        },
    //    };
    //}

    //clean_up_terminal(terminal)?;
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
