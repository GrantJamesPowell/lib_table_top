use crate::game_runner::Actions;
use crate::play::ActionResponse;
use crate::{Play, Player, PlayerSet};

#[derive(Debug, Clone)]
pub struct Turn<T: Play> {
    players_acting: PlayerSet,
    actions: Actions<T>,
}

impl<T: Play> From<PlayerSet> for Turn<T> {
    fn from(players_acting: PlayerSet) -> Self {
        Self {
            players_acting,
            actions: Default::default(),
        }
    }
}

impl<T: Play> Turn<T> {
    /// Turns the `Turn` into `Actions`
    ///
    /// ```
    /// use lttcore::examples::{GuessTheNumber, guess_the_number::Guess};
    /// use lttcore::{Turn, PlayerSet, Player, number_of_players::TWO_PLAYER};
    ///
    /// let mut turn: Turn<GuessTheNumber> = TWO_PLAYER.player_set().into();
    /// let guess: Guess = 42.into();
    /// let p1: Player = 1.into();
    /// turn.add_action(p1, guess);
    /// assert_eq!(turn.into_actions(), [(p1, guess.into())].into());
    /// ```
    pub fn into_actions(self) -> Actions<T> {
        self.actions
    }

    /// Returns the `PlayerSet` of all the players who have already submitted
    ///
    /// ```
    /// use lttcore::examples::{GuessTheNumber, guess_the_number::Guess};
    /// use lttcore::{Turn, PlayerSet, Player, number_of_players::TWO_PLAYER};
    ///
    /// let mut turn: Turn<GuessTheNumber> = TWO_PLAYER.player_set().into();
    /// assert_eq!(turn.players_who_have_submitted(), PlayerSet::empty());
    ///
    /// let p1: Player = 1.into();
    /// let guess: Guess = 42.into();
    /// turn.add_action(p1, guess);
    ///
    /// assert_eq!(turn.players_who_have_submitted(), p1.into());
    /// ```
    pub fn players_who_have_submitted(&self) -> PlayerSet {
        self.actions.iter().map(|(p, _)| *p).collect()
    }

    /// Returns a `PlayerSet` of all the players who still need to submit their input
    ///
    /// ```
    /// use lttcore::examples::{GuessTheNumber, guess_the_number::Guess};
    /// use lttcore::{Turn, PlayerSet, Player, number_of_players::TWO_PLAYER};
    ///
    /// let mut turn: Turn<GuessTheNumber> = TWO_PLAYER.player_set().into();
    /// assert_eq!(turn.unaccounted_for_players(), TWO_PLAYER.player_set());
    ///
    /// let guess: Guess = 42.into();
    /// turn.add_action(0, guess);
    ///
    /// let p1: Player = 1.into();
    /// assert_eq!(turn.unaccounted_for_players(), p1.into());
    /// ```
    pub fn unaccounted_for_players(&self) -> PlayerSet {
        self.players_acting
            .difference(self.players_who_have_submitted())
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
    /// let mut turn = game.turn();
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
            self.players_acting.contains(player),
            "{:?} was added to turn, but the player doesn't need to act this turn",
            player,
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
        self.unaccounted_for_players().is_empty()
    }
}
