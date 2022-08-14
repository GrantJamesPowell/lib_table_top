use crate::play::View;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSecretInfoUpdate;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerSecretInfo {}

impl View for PlayerSecretInfo {
    type Update = PlayerSecretInfoUpdate;
}
