use crate::networking::messages::{AuthResultMsg, GameSetupResultMsg, PingMsg, ServerInGameMsg};
use crate::Play;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ServerMsg<T: Play> {
    Ping(PingMsg),
    AuthResult(AuthResultMsg),
    GameSetupResult(GameSetupResultMsg),
    InGame(ServerInGameMsg<T>),
}

impl<T: Play> From<PingMsg> for ServerMsg<T> {
    fn from(ping: PingMsg) -> Self {
        ServerMsg::Ping(ping)
    }
}
