use crate::networking::messages::{AuthMsg, ClientInGameMsg, GameSetupMsg, PingMsg};
use crate::Play;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound = "")]
pub enum ClientMsg<T: Play> {
    Ping(PingMsg),
    Auth(AuthMsg),
    GameSetup(GameSetupMsg<T>),
    InGame(ClientInGameMsg<T>),
}
