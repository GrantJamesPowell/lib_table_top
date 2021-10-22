use crate::{Play, Player};
use std::error::Error;

pub trait Bot<T: Play> {
    fn run(
        player: Player,
        settings: &<T as Play>::Settings,
        player_view: &<T as Play>::PlayerView,
        spectator_view: &<T as Play>::SpectatorView,
    ) -> Result<<T as Play>::Action, Box<dyn Error>>;
}
