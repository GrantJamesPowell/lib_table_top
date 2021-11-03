use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PingMsg {
    Ping,
    Pong,
}

use PingMsg::*;

impl PingMsg {
    /// Opposite of {Ping/Pong}
    ///
    /// ```
    /// use lttcore::networking::messages::PingMsg::*;
    ///
    /// assert_eq!(Ping.opposite(), Pong);
    /// assert_eq!(Pong.opposite(), Ping);
    /// ```
    pub fn opposite(&self) -> Self {
        match self {
            Ping => Pong,
            Pong => Ping,
        }
    }
}
