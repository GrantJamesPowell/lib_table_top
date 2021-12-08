//! Functionality around machine readable scoring
use crate::utilities::PlayerIndexedData as PID;

/// Whether automated tooling should interpret a high score as being more favorable than a low
/// score or vice versa. Defaults to `ScoreInterpertation::HigherIsBetter`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoreInterpertation {
    /// Examples of games where higher scores are better (`ScoreInterpertation::HigherIsBetter`)
    /// * Bowling
    /// * Basketball
    /// * Gin
    HigherIsBetter,
    /// Examples of games where lower scores are better (`ScoreInterpertation::LowerIsBetter`)
    ///  * Golf
    ///  * Hearts
    ///  * [`GuessTheNumber`](crate::examples::guess_the_number)
    LowerIsBetter,
}

/// Machine readable scores
pub trait Score {
    /// See documentation for [`ScoreInterpertation`], defaults to [`ScoreInterpertation::HigherIsBetter`]
    fn score_interpertation() -> ScoreInterpertation {
        ScoreInterpertation::HigherIsBetter
    }

    /// Whether the score should be displayed directly to humans. Some games have rankings but
    /// don't have a "score" per se. `lttcore::examples::TicTacToe` is a good example where
    /// winning is a binary concept. `TicTacToe` represents the winner's score as `1` and the
    /// loser's score as `0` with `ScoreInterpertation::HigherIsBetter`. This settings tells
    /// display tools not to show the `1` and `0` of the scores, but just show the ranks if they're
    /// applicable
    fn is_score_human_interpertable() -> bool {
        true
    }

    /// The score of the game. Maps players to a "score" in whatever way is meaningful to the game.
    /// Players are interperted to be "tied" if they have the same score. If it's too early to
    /// provide a meaningful score or rankings, return `None`
    fn score(&self) -> Option<PID<i64>>;
}
