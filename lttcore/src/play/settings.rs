use crate::NumberOfPlayers;
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub trait LttSettings:
    Clone + Debug + PartialEq + Eq + Default + Serialize + DeserializeOwned
{
    fn num_players(&self) -> NumberOfPlayers;
}
