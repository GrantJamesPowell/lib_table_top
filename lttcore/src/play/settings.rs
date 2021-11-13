use crate::NumberOfPlayers;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

pub trait LttSettings:
    Clone + Debug + Default + PartialEq + Eq + Sync + Send + Serialize + DeserializeOwned + 'static
{
    fn number_of_players(&self) -> NumberOfPlayers;
    fn game_modes() -> &'static HashMap<&'static str, Arc<Self>>;
}
