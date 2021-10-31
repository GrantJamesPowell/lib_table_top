use crate::play::{ActionResponse, Actions};
use crate::{Play, Player, PlayerSet};

#[derive(Debug, Clone)]
pub struct ActionCollector<T: Play> {
    players_acting: PlayerSet,
    actions: Actions<T>,
}

impl<T: Play> From<PlayerSet> for ActionCollector<T> {
    fn from(players_acting: PlayerSet) -> Self {
        Self {
            players_acting,
            actions: Default::default(),
        }
    }
}

impl<T: Play> ActionCollector<T> {
    pub fn take_actions(&mut self) -> Actions<T> {
        std::mem::take(&mut self.actions)
    }

    pub fn all_players(&self) -> PlayerSet {
        self.players_acting
    }

    pub fn players_who_have_submitted(&self) -> PlayerSet {
        self.actions.iter().map(|(p, _)| *p).collect()
    }

    pub fn unaccounted_for_players(&self) -> PlayerSet {
        self.players_acting
            .difference(self.players_who_have_submitted())
    }

    pub fn add_action(
        &mut self,
        player: impl Into<Player>,
        action_response: impl Into<ActionResponse<<T as Play>::Action>>,
    ) {
        let player = player.into();
        let action_response = action_response.into();

        assert!(
            self.players_acting.contains(player),
            "{:?} was added to action_collector, but the player doesn't need to act this turn",
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

    pub fn is_ready(&self) -> bool {
        self.unaccounted_for_players().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examples::{guess_the_number::Guess, GuessTheNumber};
    use crate::{number_of_players::TWO_PLAYER, GameProgressionBuilder, Player, PlayerSet};

    #[test]
    fn test_you_can_turn_action_collector_take_actions() {
        let mut action_collector: ActionCollector<GuessTheNumber> = TWO_PLAYER.player_set().into();
        let guess: Guess = 42.into();
        let p1: Player = 1.into();
        action_collector.add_action(p1, guess);
        assert_eq!(action_collector.take_actions(), [(p1, guess.into())].into());
    }

    #[test]
    fn test_you_can_get_already_submitted_players() {
        let mut action_collector: ActionCollector<GuessTheNumber> = TWO_PLAYER.player_set().into();
        assert_eq!(
            action_collector.players_who_have_submitted(),
            PlayerSet::empty()
        );

        let p1: Player = 1.into();
        let guess: Guess = 42.into();
        action_collector.add_action(p1, guess);

        assert_eq!(action_collector.players_who_have_submitted(), p1.into());
    }

    #[test]
    fn test_you_can_get_players_who_still_need_to_submit_input() {
        let mut action_collector: ActionCollector<GuessTheNumber> = TWO_PLAYER.player_set().into();
        assert_eq!(
            action_collector.unaccounted_for_players(),
            TWO_PLAYER.player_set()
        );

        let guess: Guess = 42.into();
        action_collector.add_action(0, guess);

        let p1: Player = 1.into();
        assert_eq!(action_collector.unaccounted_for_players(), p1.into());
    }

    #[test]
    #[should_panic(
        expected = "Player(255) was added to action_collector, but the player doesn't need to act this turn"
    )]
    fn test_it_panics_when_trying_to_add_a_player_whos_not_in_the_set() {
        let player: Player = 255.into();
        let game = GameProgressionBuilder::<GuessTheNumber>::default()
            .build()
            .unwrap();
        let mut action_collector: ActionCollector<GuessTheNumber> =
            game.which_players_input_needed().into();

        let guess: Guess = 42.into();
        action_collector.add_action(player, guess);
    }
}
