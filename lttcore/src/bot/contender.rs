use super::StatefulBot;
use crate::play::Play;
use std::borrow::Cow;
use std::fmt::Display;
use std::sync::Arc;

/// Produce owned instances of [`StatefulBot`] trait objects from a "template" [`StatefulBot`]
///
/// Most of the use cases for [`StatefulBot`]/[`Bot`](super::Bot) revolve around being able to use instances of
/// it as trait objects. Like having a `PlayerIndexedData<Box<dyn StatefulBot<T>>>` to represent a
/// group of (likely heterogenous) bots playing a game. This works out fine until your use case is
/// that you want to have the same bot play in multiple seperate games (see the "stadium" crate).
/// This type works around the fact that you can't use [`Clone`] in trait objects to make copies of
/// a bot
#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct Contender<T: Play> {
    name: Cow<'static, str>,
    bot: Arc<dyn MakeStatefulBotInstance<Game = T>>,
}

impl<T: Play> Contender<T> {
    /// Create a [`Contender`] from a [`Bot`](super::Bot) or [`StatefulBot`]
    /// ```
    /// use lttcore::bot::Contender;
    /// use lttcore::examples::{TicTacToe, tic_tac_toe::{
    ///   TicTacToeBotWrapper, bot::prebuilt::Intermediate
    /// }};
    ///
    /// let bot = TicTacToeBotWrapper(Intermediate);
    /// let contender = Contender::new(bot);
    /// assert_eq!(contender.name(), "Intermediate");
    /// ```
    pub fn new(bot: impl StatefulBot<Game = T> + Clone + Display) -> Self {
        let name = format!("{}", bot);
        Self::new_with_name(bot, name)
    }

    /// Create a [`Contender`] with a custom name
    /// ```
    /// use lttcore::bot::Contender;
    /// use lttcore::examples::{TicTacToe, tic_tac_toe::{
    ///   TicTacToeBotWrapper, bot::prebuilt::Intermediate
    /// }};
    ///
    /// let bot = TicTacToeBotWrapper(Intermediate);
    /// let contender = Contender::new_with_name(bot, "Custom Name");
    /// assert_eq!(contender.name(), "Custom Name");
    /// ```
    pub fn new_with_name(
        bot: impl StatefulBot<Game = T> + Clone,
        name: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            name: name.into(),
            bot: Arc::new(bot),
        }
    }

    /// Return the name of the [`Contender`]
    ///
    /// ```
    /// use lttcore::bot::Contender;
    /// use lttcore::examples::{TicTacToe, tic_tac_toe::{
    ///   TicTacToeBotWrapper,
    ///   bot::prebuilt::{Intermediate, Expert}
    /// }};
    ///
    /// // Use the default `Display` impl for a name
    /// let bot = TicTacToeBotWrapper(Intermediate);
    ///
    /// let contender = Contender::new(bot.clone());
    /// assert_eq!(contender.name(), "Intermediate");
    ///
    /// // Use a custom set name
    /// let contender = Contender::new_with_name(bot.clone(), "Custom Name");
    /// assert_eq!(contender.name(), "Custom Name");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Create an instance of a `Box<dyn StatefulBot<T>>` for this [`Contender`]
    ///
    /// Will be a clone of the [`StatefulBot`] provided to the [`Contender::new`] constructor
    pub fn make_stateful_bot_instance(&self) -> Box<dyn StatefulBot<Game = T>> {
        self.bot.make_stateful_bot_instance()
    }

    /// A debugging tool for trying to figure out what bot this instance of contender is associated
    /// with
    ///
    /// # Warnings
    ///
    /// Due to tradeoffs in Rust's type system, It's *really* hard to go backwards from a `dyn
    /// Trait` to the concrete type that implemented `Trait`. This function relies on
    /// [`std::any::type_name`] which makes no promises about the accuracy or stability of the
    /// output. Use this function as a debugging tool, but not to generate source code. Do not
    /// depend on it's output
    pub fn source_hint(&self) -> &'static str {
        self.bot.source_hint()
    }
}

/// Trait to generate owned instances of `dyn StatefulBot`
///
/// This is blanked implmented for all [`StatefulBot`]
///
/// # Implementation notes
///
/// This trait is mostly a workaround for the fact that we can't use `Clone` in trait objects. The
/// behaviour of implementations of this trait _should_ be to produce identical instances
/// everytime, but we can't enforce that in the type system, so we need to enforce it via
/// convention
trait MakeStatefulBotInstance: Send + Sync + 'static {
    /// The game that this trait produces [`StatefulBot`]s for.
    type Game: Play;

    /// Generate a new owned trait object implmenting [`StatefulBot`]
    fn make_stateful_bot_instance(&self) -> Box<dyn StatefulBot<Game = Self::Game>>;

    /// Helper function to generate better crash reports
    ///
    /// This defaults to [`std::any::type_name`] which is a crapshoot and shouldn't be relied upon,
    /// it's also the best we have 🤷
    ///
    /// We can get the state of the bot via the [`SerializeSelf`] trait, but it's really difficult
    /// to go backwards to the original struct implementing `StatefulBot` when we're using the
    /// trait through `dyn` (most of the time). This is a hack to provide slightly better error
    /// messages
    fn source_hint(&self) -> &'static str {
        "<No Source Hint Available>"
    }
}

impl<Bot: StatefulBot + Clone> MakeStatefulBotInstance for Bot {
    type Game = Bot::Game;

    fn make_stateful_bot_instance(&self) -> Box<dyn StatefulBot<Game = Self::Game>> {
        Box::new(self.clone())
    }

    fn source_hint(&self) -> &'static str {
        std::any::type_name::<Bot>()
    }
}
