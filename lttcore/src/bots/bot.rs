use crate::{Play, Player, Spectator};
use std::error::Error;

pub trait Bot<T: Play> {
    fn run(
        player: Player,
        spectator: &Spectator<T>,
        player_view: &<T as Play>::PlayerView,
    ) -> Result<<T as Play>::Action, Box<dyn Error>>;
}
