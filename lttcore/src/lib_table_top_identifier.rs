/// Lib Table Top Identifiers
///
/// Trait to allow higher level tooling to introspect on the games it supports
pub trait LibTableTopIdentifier {
    /// This method allows higher level tooling to disambiguate between items in "stringly" typed
    /// contexts (i.e. Client and Server protocol negotiation where the server may support more or
    /// fewer games/bots/etc than the client). It's important that
    /// [`lib_table_top_identifier`](LibTableTopIdentifier::lib_table_top_identifier) is constant for your
    /// game as long as it's conceptually the same. Most tooling will not be able support multiple
    /// games with the same value
    /// [`lib_table_top_identifier`](LibTableTopIdentifier::lib_table_top_identifier). The value provided by
    /// this trait should should likely just be the string literal version of whatever you're
    /// identifying
    ///
    /// ```
    /// use lttcore::examples::{TicTacToe, GuessTheNumber};
    /// use lttcore::LibTableTopIdentifier;
    ///
    /// assert_eq!(TicTacToe::lib_table_top_identifier(), "TicTacToe");
    /// assert_eq!(GuessTheNumber::lib_table_top_identifier(), "GuessTheNumber");
    /// ```
    fn lib_table_top_identifier() -> &'static str;
}
