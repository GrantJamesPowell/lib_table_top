use crate::networking::Token;
use crate::Play;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum GameSetupMsg<T: Play> {
    CreateGame { settings: <T as Play>::Settings },
    JoinGame { game_id: Uuid, token: Option<Token> },
    JoinMatchMaker { lobby: Uuid, token: Option<Token> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameSetupResultMsg {
    GameJoined { game_id: Uuid },
    InvalidToken,
    AuthorizationRequired,
    GameFull,
}
