use super::channels::{ToGameHostMsgReceiver, ToObserverMsgSender, ToPlayerMsgSender};
use crate::messages::{ToGameHostMsg::*, ToObserverMsg, ToPlayerMsg};
use lttcore::play::{ActionResponse, EnumeratedGameAdvance, Play};
use lttcore::pov::game_progression::GameProgression;
use lttcore::utilities::{PlayerIndexedData as PID, PlayerItemCollector as PIC};

pub async fn game_host<T: Play>(
    mut game: GameProgression<T>,
    mut mailbox: ToGameHostMsgReceiver<T>,
    to_players: PID<ToPlayerMsgSender<T>>,
    to_observer: ToObserverMsgSender<T>,
) -> GameProgression<T> {
    while !game.is_concluded() {
        let mut returned_actions: PIC<ActionResponse<T>> = game.which_players_input_needed().into();

        while !returned_actions.unaccounted_for_players().is_empty() {
            match mailbox.recv().await {
                None => return game,
                Some(msg) => match msg {
                    RequestObserverState => {
                        let msg = ToObserverMsg::SyncState(game.game_observer());
                        to_observer
                            .send(msg)
                            .expect("observer connections multiplexer is still alive");
                    }
                    RequestPlayerState { player } => {
                        let game_player = game.game_player(player);
                        to_players[player]
                            .send(ToPlayerMsg::SyncState(game_player))
                            .expect("player connections multiplexer is still alive");
                    }
                    SubmitActionResponse { player, response } => {
                        returned_actions.add(player, response);
                    }
                },
            }
        }

        let game_advance = game.submit_actions(returned_actions.into_items());
        send_update(game_advance, &to_players, &to_observer).await;
    }

    game
}

async fn send_update<T: Play>(
    update: EnumeratedGameAdvance<T>,
    to_players: &PID<ToPlayerMsgSender<T>>,
    to_observer: &ToObserverMsgSender<T>,
) {
    let observer_update = update.observer_update().into_owned();
    to_observer
        .send(ToObserverMsg::Update(observer_update))
        .unwrap();

    for (player, to_player) in to_players.iter() {
        let player_update = update.player_update(player).into_owned();
        to_player.send(ToPlayerMsg::Update(player_update)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lttcore::examples::{
        guess_the_number::{Guess, Settings},
        GuessTheNumber,
    };
    use lttcore::play::ActionResponse::Response;
    use lttcore::utilities::PlayerIndexedData;
    use tokio::sync::mpsc::unbounded_channel;

    #[tokio::test]
    async fn test_game_host_returns_when_all_mailbox_senders_are_dropped() {
        let settings: Settings = Default::default();
        let game = GameProgression::from_settings(settings);

        let (to_mailbox, mailbox) = unbounded_channel();
        let (to_observer, _) = unbounded_channel();
        let to_players = PlayerIndexedData::init_with(game.players(), |_| {
            let (to_player, _) = unbounded_channel();
            to_player
        });

        let handle = tokio::spawn(game_host::<GuessTheNumber>(
            game.clone(),
            mailbox,
            to_players,
            to_observer,
        ));

        drop(to_mailbox);
        assert_eq!(handle.await.unwrap(), game);
    }

    #[tokio::test]
    async fn test_game_host_returns_with_a_completed_progression() {
        let settings: Settings = Default::default();
        let mut game = GameProgression::from_settings(settings);
        let guess: Guess = 0.into();

        game.submit_actions([(0.into(), Response(guess))]);

        let (to_mailbox, mailbox) = unbounded_channel();
        let (to_observer, _) = unbounded_channel();
        let to_players = PlayerIndexedData::init_with(game.players(), |_| {
            let (to_player, _) = unbounded_channel();
            to_player
        });

        let handle = tokio::spawn(game_host::<GuessTheNumber>(
            game.clone(),
            mailbox,
            to_players,
            to_observer,
        ));

        assert_eq!(handle.await.unwrap(), game);
        // Make sure the sender doesn't drop until the game returns
        // because that signals to the game to stop
        drop(to_mailbox);
    }
}
