use crate::play::{settings::NumPlayers, GameState, NumberOfPlayers, Play, Player, TurnNum};
use crate::pov::game_progression::GameProgression;

impl<T: Play> GameProgression<T> {
    pub fn is_concluded(&self) -> bool {
        self.game_state.action_requests.is_none()
    }

    pub fn turn_num(&self) -> TurnNum {
        self.turn_num
    }

    pub fn game_state(&self) -> &GameState<T> {
        &self.game_state
    }

    pub fn settings(&self) -> &T::Settings {
        &self.settings
    }

    pub fn public_info(&self) -> &T::PublicInfo {
        &self.game_state.public_info
    }

    pub fn player_secret_info(&self, player: Player) -> &T::PlayerSecretInfo {
        &self.game_state.player_secret_info[player]
    }

    pub fn number_of_players(&self) -> NumberOfPlayers {
        self.settings().number_of_players()
    }

    pub fn players(&self) -> impl Iterator<Item = Player> + '_ {
        self.number_of_players().players()
    }

    pub fn player_phases(&self) -> impl Iterator<Item = (Player, &T::Phase)> + '_ {
        self.game_state
            .action_requests
            .iter()
            .flat_map(|ps| ps.iter())
    }

    pub fn which_players_input_needed(&self) -> impl Iterator<Item = Player> + '_ {
        self.game_state
            .action_requests
            .iter()
            .flat_map(|ps| ps.players())
    }
}
