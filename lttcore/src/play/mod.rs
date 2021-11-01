mod game_advance;
mod play;
mod turn_num;

pub mod view;

pub mod settings {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
    pub struct NoCustomSettings;
}

pub use view::View;

pub use turn_num::TurnNum;

pub use game_advance::{EnumeratedGameAdvance, GameAdvance};

pub use play::{
    ActionResponse, Actions, DebugMsg, DebugMsgs, Play, PlayerSecretInfoUpdates, PlayerSecretInfos,
};
