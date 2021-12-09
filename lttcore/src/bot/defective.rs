/// Helper macro to create bots that implement [`Bot`](super::Bot) for a particular game, but
/// always panic when called. This is useful for testing tooling that needs to handle bot panics
macro_rules! panicking_bot {
    ($game:ty) => {
        ::paste::paste! {
            #[doc = "A bot designed to play "]
            #[doc = stringify!($game)]
            #[doc = " but will panic at every opportunity, useful for testing tools that need to handle bot panics"]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ::serde::Serialize, ::serde::Deserialize)]
            pub struct [<$game PanicBot>];

            impl $crate::bot::Bot for [<$game PanicBot>] {
                type Game = $game;

                fn on_action_request(
                    &self,
                    _pov: &$crate::pov::player::PlayerPov<'_, Self::Game>,
                    _phase: &<$game as $crate::play::Play>::Phase,
                    _seed: &$crate::play::Seed
                    ) -> <Self::Game as $crate::play::Play>::Action {
                    panic!("Bot intentionally panicked")
                }
            }
        }
    }
}

pub(crate) use panicking_bot;
