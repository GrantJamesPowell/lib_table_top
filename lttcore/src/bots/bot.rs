use crate::{Play, Player, Spectator};
use std::error::Error;

pub trait Bot<T: Play> {
    fn run(
        player: Player,
        spectator: &Spectator<T>,
        secret_info: &<T as Play>::PlayerSecretInfo,
    ) -> Result<<T as Play>::Action, Box<dyn Error>>;
}
