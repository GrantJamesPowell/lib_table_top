use super::channels::{ToGameHostMsgReceiver, ToObserverMsgSender, ToPlayerMsgSender};
use crate::messages::{ToGameHostMsg::*, ToObserverMsg, ToPlayerMsg};
use lttcore::play::{ActionResponse, Play};
use lttcore::pov::game_progression::GameProgression;
use lttcore::utilities::{PlayerIndexedData as PID, PlayerItemCollector as PIC, PlayerSet};

pub async fn game_host<T: Play>(
    mut game: GameProgression<T>,
    mut mailbox: ToGameHostMsgReceiver<T>,
    to_players: PID<ToPlayerMsgSender<T>>,
    to_observer: ToObserverMsgSender<T>,
) -> GameProgression<T> {
    while !game.is_concluded() {
        let mut returned_actions: PIC<ActionResponse<T>> = game
            .which_players_input_needed()
            .collect::<PlayerSet>()
            .into();

        while !returned_actions.are_all_players_accounted_for() {
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

        let update = game.resolve(returned_actions.into_items().collect());

        let observer_update = update.observer_update().into_owned();
        let _maybe_send_error = to_observer.send(ToObserverMsg::Update(observer_update));

        for (player, to_player) in to_players.iter() {
            let player_update = update.player_update(player).into_owned();
            let _maybe_send_error = to_player.send(ToPlayerMsg::Update(player_update));
        }

        game.update(update);
    }

    game
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

        let to_players: PlayerIndexedData<_> = game
            .players()
            .map(|player| {
                let (to_player, _) = unbounded_channel();
                (player, to_player)
            })
            .collect();

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
        let actions = game
            .which_players_input_needed()
            .map(|player| (player, Response(Guess::from(u32::from(player)))))
            .collect();

        let update = game.resolve(actions);
        game.update(update);

        let (to_mailbox, mailbox) = unbounded_channel();
        let (to_observer, _) = unbounded_channel();
        let to_players = game
            .players()
            .map(|player| {
                let (to_player, _) = unbounded_channel();
                (player, to_player)
            })
            .collect();

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
