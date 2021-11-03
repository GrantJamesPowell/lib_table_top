use crate::{GamePlayer, Play, TurnNum};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token(#[serde(with = "hex")] [u8; 32]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PingMsg {
    Ping,
    Pong,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthMsg {
    TokenAuth(Token),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthResultMsg {
    Authorized { user_name: String, user_id: Uuid },
    Unauthorized { msg: String },
    ConnectionLimitExceeded,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ClientInGameMsg<T: Play> {
    RequestStateSync,
    SubmitAction {
        turn: TurnNum,
        action: <T as Play>::Action,
    },
    Resign,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ServerInGameMsg<T: Play> {
    Ping(PingMsg),
    StateSync(GamePlayer<T>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ClientMsg<T: Play> {
    Ping(PingMsg),
    Auth(AuthMsg),
    GameSetup(GameSetupMsg<T>),
    InGame(ClientInGameMsg<T>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ServerMsg<T: Play> {
    Ping(PingMsg),
    AuthResult(AuthResultMsg),
    GameSetupResult(GameSetupResultMsg),
    InGame(ServerInGameMsg<T>),
}
