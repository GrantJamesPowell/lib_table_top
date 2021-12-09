use super::Bot;
use crate::play::Play;
use std::borrow::Cow;
use std::fmt::Display;
use std::sync::Arc;

/// Produce owned instances of [`Bot`] trait objects from a "template" [`Bot`]
///
/// Most of the use cases for [`Bot`]/[`Bot`](super::Bot) revolve around being able to use instances of
/// it as trait objects. Like having a `PlayerIndexedData<Box<dyn Bot<T>>>` to represent a
/// group of (likely heterogenous) bots playing a game. This works out fine until your use case is
/// that you want to have the same bot play in multiple seperate games (see the "stadium" crate).
/// This type works around the fact that you can't use [`Clone`] in trait objects to make copies of
/// a bot
#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct Contender<T: Play> {
    name: Cow<'static, str>,
    bot: Arc<dyn MakeBotInstance<Game = T>>,
}

impl<T: Play, B: Bot<Game = T> + Clone + Display> From<B> for Contender<T> {
    fn from(bot: B) -> Self {
        Self::new(bot)
    }
}

impl<T: Play, N: Into<Cow<'static, str>>, B: Bot<Game = T> + Clone + Display> From<(N, B)>
    for Contender<T>
{
    fn from((name, bot): (N, B)) -> Self {
        Self::new_with_name(bot, name)
    }
}

impl<T: Play> Contender<T> {
    /// Create a [`Contender`] from a [`Bot`](super::Bot) or [`Bot`]
    /// ```
    /// use lttcore::bot::Contender;
    /// use lttcore::examples::{TicTacToe, tic_tac_toe::{
    ///   TicTacToeBot, bot::prebuilt::IntermediateSkill
    /// }};
    ///
    /// let contender = Contender::new(IntermediateSkill.into_bot());
    /// assert_eq!(contender.name(), "IntermediateSkill");
    /// ```
    pub fn new(bot: impl Bot<Game = T> + Clone + Display) -> Self {
        let name = format!("{}", bot);
        Self::new_with_name(bot, name)
    }

    /// Create a [`Contender`] with a custom name
    /// ```
    /// use lttcore::bot::Contender;
    /// use lttcore::examples::{TicTacToe, tic_tac_toe::{
    ///   TicTacToeBot, bot::prebuilt::IntermediateSkill
    /// }};
    ///
    /// let contender = Contender::new_with_name(
    ///   IntermediateSkill.into_bot(),
    ///   "Custom Name"
    /// );
    /// assert_eq!(contender.name(), "Custom Name");
    /// ```
    pub fn new_with_name(
        bot: impl Bot<Game = T> + Clone,
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
    ///   TicTacToeBot,
    ///   bot::prebuilt::{IntermediateSkill, ExpertSkill}
    /// }};
    ///
    /// // Use the default `Display` impl for a name
    /// let contender = Contender::new(IntermediateSkill.into_bot());
    /// assert_eq!(contender.name(), "IntermediateSkill");
    ///
    /// // Use a custom set name
    /// let contender = Contender::new_with_name(
    ///   IntermediateSkill.into_bot(),
    ///   "Custom Name"
    /// );
    /// assert_eq!(contender.name(), "Custom Name");
    /// ```
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Create an instance of a `Box<dyn Bot<T>>` for this [`Contender`]
    ///
    /// Will be a clone of the [`Bot`] provided to the [`Contender::new`] constructor
    pub fn make_bot_instance(&self) -> Box<dyn Bot<Game = T>> {
        self.bot.make_bot_instance()
    }
}

/// Trait to generate owned instances of `dyn Bot`
///
/// This is blanked implmented for all [`Bot`]
///
/// # Implementation notes
///
/// This trait is mostly a workaround for the fact that we can't use `Clone` in trait objects. The
/// behaviour of implementations of this trait _should_ be to produce identical instances
/// everytime, but we can't enforce that in the type system.
trait MakeBotInstance: Send + Sync + 'static {
    /// The game that this trait produces [`Bot`]s for.
    type Game: Play;

    /// Generate a new owned trait object implmenting [`Bot`]
    fn make_bot_instance(&self) -> Box<dyn Bot<Game = Self::Game>>;
}

impl<B: Bot + Clone> MakeBotInstance for B {
    type Game = B::Game;

    fn make_bot_instance(&self) -> Box<dyn Bot<Game = Self::Game>> {
        Box::new(self.clone())
    }
}
