use serde::{Deserialize, Serialize};
use crate::play::View;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSecretInfoUpdate;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSecretInfo;

impl View for PlayerSecretInfo {
    type Update = PlayerSecretInfoUpdate;
}
