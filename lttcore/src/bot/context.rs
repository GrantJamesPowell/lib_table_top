use super::BotError;
use crate::{
    id::GameId,
    play::{Play, Seed, TurnNum},
};
use std::time::Duration;

/// Struct representing the "context" that the [`Bot`](super::Bot) is executing in
#[allow(missing_debug_implementations)]
#[non_exhaustive]
#[derive(Builder)]
pub struct BotContext<'a, T: Play> {
    /// The seed for the [`Bot`](super::Bot) instance
    ///
    /// Notes:
    /// * This should *_not_* be the same seed the underlying [`GameProgression`](crate::pov::game_progression::GameProgression) is using.
    /// * Each [`Bot`](super::Bot) within a game should have a different [`Seed`]
    /// * The [`BotContext::rng_for_turn`] method uses the [`TurnNum`] to give a different `Rng`
    /// per turn, so the same seed can be reused for the same bot within the same game
    seed: &'a Seed,
    /// The turn number of the game
    turn_num: TurnNum,
    /// The [`GameId`] for the current Game.
    ///
    /// This likely isn't useful execept in the very specific case of you'd like to build a
    /// [`Bot`](super::Bot) that "cheats" by sharing information with other bots via side channels.
    ///
    /// If you're building a runtime that provides a [`BotContext`] provide this value correctly if
    /// possible, because who are we to ruin the fun
    #[builder(setter(strip_option), default = "None")]
    game_id: Option<&'a GameId>,
    /// A function that calculates the "amount of time remaining."
    ///
    /// If you're building a networked client, this should roughly coincide with the amount of time
    /// the server has given for the action minus the network latency plus a safety factor.
    ///
    /// This is more a _hint_ to the [`Bot`](super::Bot) as we don't have a good way to preempt the
    /// bot in the case it's using too much time. In a language like `Erlang` we would be able to
    /// do it, but not with cooperative green threads.
    #[builder(default = "&time_remaining_default")]
    time_remaining: &'a dyn Fn() -> Option<Duration>,
    #[builder(default, setter(skip))]
    _marker: std::marker::PhantomData<fn() -> T>,
}

impl<'a, T: Play> BotContext<'a, T> {
    /// The [`Seed::rng_for_turn`] for this turn.
    pub fn rng_for_turn(&self) -> impl rand::Rng {
        self.seed.rng_for_turn(self.turn_num)
    }

    /// The [`GameId`] (if available...). This likely isn't useful execept in the very specific
    /// case of you'd like to build a [`Bot`](super::Bot) that "cheats" by sharing information with
    /// other bots via side channels
    pub fn game_id(&self) -> Option<&'a GameId> {
        self.game_id
    }

    /// The current [`TurnNum`]
    pub fn turn_num(&self) -> TurnNum {
        self.turn_num
    }

    /// Advanced feature to tell the runtime to "checkpoint" it's current best action
    ///
    /// This workflow is for advanced bots playing in games with time constraints, this function
    /// allows the bot to tell the runtime "I have a decent answer, submit this if I'm out of time,
    /// else let me continue to refine my answer"
    ///
    /// The psuedocode for this workflow looks something like this
    ///
    /// ```ignore
    /// let mut action = some_fast_method_to_generate_an_action();
    /// checkpoint(&action)?;
    ///  
    /// for _ in 0..100_000 {
    ///    action = refine_action(action);
    ///    checkpoint(&action)?;
    /// }
    ///
    /// action
    /// ```
    pub fn checkpoint(&self, action: &T::Action) -> Result<(), BotError<T>> {
        if let Some(time_remaining) = self.time_remaining() {
            if time_remaining.is_zero() {
                return Err(BotError::TimeExceeded(Some(action.clone())));
            }
        }

        Ok(())
    }

    /// Returns the amount of time remaining to your bot.
    ///
    /// Should be interperted as follows
    /// * `None` => There is no time limit, feel free to run as long as you'd like
    /// * `Some(duration) when duration.is_zero()` => Bot's time is up
    /// * `Some(duration)` => bot has `duration` left to complete it's work
    pub fn time_remaining(&self) -> Option<Duration> {
        (self.time_remaining)()
    }
}

fn time_remaining_default() -> Option<Duration> {
    None
}
