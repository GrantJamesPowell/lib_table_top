/// Lib Table Top Identifiers
///
/// Trait to allow higher level tooling to introspect on the games it supports
pub trait LttVersion {
    /// This method allows higher level tooling to disambiguate between different games in
    /// "stringly" typed contexts (i.e. Client and Server protocol negotiation where the server may
    /// support more or fewer games than the client). It's important that
    /// [`lib_table_top_identifier`](LttVersion::lib_table_top_identifier) is constant for your
    /// game as long as it's conceptually the same. Most tooling will not be able support multiple
    /// games with the same value [`lib_table_top_identifier`](LttVersion::lib_table_top_identifier). The
    /// value provided by this trait should should likely just be the string literal version of
    /// your game's name.
    ///
    /// ```
    /// use lttcore::examples::TicTacToe;
    /// use lttcore::play::LttVersion;
    /// assert_eq!(TicTacToe::lib_table_top_identifier(), "TicTacToe");
    /// ```
    fn lib_table_top_identifier() -> &'static str;
}
