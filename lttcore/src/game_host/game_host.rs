use super::action_collector::ActionCollector;
use crate::play::EnumeratedGameAdvance;
use crate::{ActionResponse, GameProgression, Play, Player};

pub struct GameHost<T: Play> {
    game_progression: GameProgression<T>,
    action_collector: ActionCollector<T>,
}

impl<T: Play> From<GameProgression<T>> for GameHost<T> {
    fn from(game_progression: GameProgression<T>) -> Self {
        let action_collector: ActionCollector<T> =
            game_progression.which_players_input_needed().into();

        Self {
            game_progression,
            action_collector,
        }
    }
}

impl<T: Play> GameHost<T> {
    pub fn from_settings(settings: impl Into<<T as Play>::Settings>) -> Self {
        let game_progression = GameProgression::from_settings(settings.into());
        game_progression.into()
    }

    pub fn into_game_progression(self) -> GameProgression<T> {
        self.game_progression
    }

    pub fn game_progression(&self) -> &GameProgression<T> {
        &self.game_progression
    }

    pub fn submit_action_response(
        &mut self,
        player: Player,
        action_response: impl Into<ActionResponse<<T as Play>::Action>>,
    ) -> Option<EnumeratedGameAdvance<T>> {
        self.action_collector.add_action(player, action_response);

        if self.action_collector.is_ready() {
            let (new_progression, game_advance) = self
                .game_progression
                .submit_actions(self.action_collector.take_actions());

            self.game_progression = new_progression;
            Some(game_advance)
        } else {
            None
        }
    }
}
