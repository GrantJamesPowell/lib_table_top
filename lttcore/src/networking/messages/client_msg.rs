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

impl<T: Play> From<PingMsg> for ClientMsg<T> {
    fn from(ping: PingMsg) -> Self {
        ClientMsg::Ping(ping)
    }
}

impl<T: Play> ClientMsg<T> {
    pub fn verify(&self, authorized: bool) -> bool {
        use ClientMsg::*;

        match self {
            Ping(_) | Auth(_) => true,
            GameSetup(_) | InGame(_) => authorized,
        }
    }
}
