use serde::{Deserialize, Serialize};
use crate::play::View;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSecretInfoUpdate;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameSecretInfo;

impl View for GameSecretInfo {
    type Update = GameSecretInfoUpdate;
}
