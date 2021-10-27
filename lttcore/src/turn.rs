use crate::game_runner::Actions;
use crate::play::ActionResponse;
use crate::{Play, Player, PlayerSet};

#[derive(Debug, Clone)]
pub struct Turn<T: Play> {
    pub(crate) turn_num: u64,
    pub(crate) action_requests: PlayerSet,
    pub(crate) actions: Actions<T>,
}

impl<T: Play> Turn<T> {
    pub fn number(&self) -> u64 {
        self.turn_num
    }

    pub fn pending_action_requests(&self) -> PlayerSet {
        self.action_requests
            .players()
            .filter(|player| {
                self.actions
                    .binary_search_by_key(&player, |(p, _)| &*p)
                    .is_err()
            })
            .collect()
    }

    /// Add an action to the turn
    ///
    /// # Panics
    ///
    /// This panics if the `Player` isn't in the turn
    /// ```should_panic
    /// use lttcore::examples::{GuessTheNumber, guess_the_number::Guess};
    /// use lttcore::{Player, GameRunnerBuilder};
    ///
    /// let player: Player = 255.into();
    /// let game = GameRunnerBuilder::<GuessTheNumber>::default().build().unwrap();
    /// let mut turn = game.turn().unwrap();
    ///
    /// let guess: Guess = 42.into();
    /// turn.add_action(player, guess);
    /// ```
    pub fn add_action(
        &mut self,
        player: impl Into<Player>,
        action_response: impl Into<ActionResponse<<T as Play>::Action>>,
    ) {
        let player = player.into();
        let action_response = action_response.into();

        assert!(
            self.action_requests.contains(player),
            "{:?} was added to turn {:?}, but player isn't in the turn",
            player,
            self.turn_num
        );

        match self.actions.binary_search_by_key(&player, |(p, _)| *p) {
            Ok(existing_action_index) => {
                self.actions[existing_action_index] = (player, action_response);
            }
            Err(index) => {
                self.actions.insert(index, (player, action_response));
            }
        }
    }

    pub fn is_ready_to_submit(&self) -> bool {
        self.pending_action_requests().is_empty()
    }
}
