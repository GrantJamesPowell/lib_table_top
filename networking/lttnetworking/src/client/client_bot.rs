use lttcore::bots::Bot;
use lttcore::{GamePlayer, Play, Seed};
use lttruntime::messages::player::{
    FromPlayerMsg::{self, *},
    SubmitActionErrorKind::*,
    ToPlayerMsg::{self, *},
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct Inbox<T: Play> {
    pub from_server: UnboundedReceiver<ToPlayerMsg<T>>,
}

pub struct Outbox<T: Play> {
    pub to_server: UnboundedSender<FromPlayerMsg<T>>,
}

pub async fn client_bot<T: Play, B: Bot<Game = T>>(
    seed: Seed,
    mut inbox: Inbox<T>,
    outbox: Outbox<T>,
) -> anyhow::Result<()> {
    let mut game_player: Option<GamePlayer<T>> = None;
    outbox.to_server.send(RequestPrimary).unwrap();

    while let Some(msg) = inbox.from_server.recv().await {
        match msg {
            SyncState(game_player_state) => {
                game_player = Some(game_player_state);
            }
            Update(update) => {
                if let Some(game_player) = game_player.as_mut() {
                    game_player.update(update);

                    if game_player.is_player_input_needed() {
                        let mut rng = seed.rng_for_turn(game_player.turn_num());
                        if let Ok(action) = B::run(&game_player.player_pov(), &mut rng) {
                            outbox.to_server.send(SubmitAction {
                                action,
                                turn: game_player.turn_num(),
                            })?;
                        }
                    }
                }
            }
            SubmitActionError(kind) => {
                match kind {
                    NotPrimary => break,
                    Timeout { .. } => {
                        // Something here?
                    }
                    InvalidTurn { .. } => {
                        // Something here?
                    }
                }
            }
            SetPrimaryStatus(true) => {
                // pass
            }
            GameOver | SetPrimaryStatus(false) => break,
        }
    }

    Ok(())
}
