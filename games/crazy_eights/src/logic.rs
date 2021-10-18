use crate::{
    Action::{self, *},
    Direction::{self, *},
    Power::*,
    Settings,
};
use lttcore::common::deck::{Card, Rank, Suit, SUITS};
use lttcore::Player;
use std::num::NonZeroU8;

/// Returns the next player given the direction and number of players,
/// will wrap the player numbers correctly.
///
/// ```
/// use crazy_eights::{Direction::*, logic::next_players_turn};
///
/// for (player, num_players, direction, expected) in [
///   // Player 0 is to the left of Player 1 in a 3 Player game
///   (1, 3, Left, 0),
///   // Player 2 is to the right of Player 1 in a 3 Player game
///   (1, 3, Right, 2),
///   // Player 0 is to the right of Player 1 in a 2 Player game
///   (1, 2, Right, 0),
///   // Player 0 is to the left of Player 1 in a 2 Player game
///   (1, 2, Left, 0),
///   // Player 0 is to the left of himself in a 1 Player game
///   (0, 1, Left, 0),
///   // Player 0 is to the right of himself in a 1 Player game
///   (0, 1, Right, 0),
/// ] {
///   assert_eq!(
///      next_players_turn(player.into(), num_players.try_into().unwrap(), direction),
///      expected.into()
///   )
/// }
///
/// ```
///
/// # Panics
///
/// Panics if the player is outside the range of number of players
///
/// ```should_panic
/// use crazy_eights::{Direction::*, logic::next_players_turn};
///
/// next_players_turn(42.into(), 4.try_into().unwrap(), Left);
/// ```
pub fn next_players_turn(
    player: Player,
    number_of_players: NonZeroU8,
    direction: Direction,
) -> Player {
    assert!(
        player.as_u8() < number_of_players.get(),
        "Player must be less than number of players"
    );

    let player_num = player.as_u8();
    match direction {
        Left => match player_num {
            0 => (number_of_players.get() - 1).into(),
            n => (n - 1).into(),
        },
        Right => match player_num {
            n if n == (number_of_players.get() - 1) => 0.into(),
            n => (n + 1).into(),
        },
    }
}

/// Return the legal actions for a card, given the current suit/rank and power rules
///
/// ```
/// use lttcore::common::deck::{cards::*, Rank::*, Suit::*};
/// use crazy_eights::{Action::*, logic::*};
///
/// let settings = Default::default();
///
/// // You can't play a card if it's suit and rank don't match the current suit and rank
///
/// assert!(
///   actions_for_card(JACK_OF_SPADES, Seven, Hearts, &settings)
///     .next()
///     .is_none()
/// );
///
/// // You can play a card if it's suit or rank match (or both!)
///
/// assert_eq!(
///   actions_for_card(JACK_OF_SPADES, Jack, Hearts, &settings).collect::<Vec<_>>(),
///   vec![PlayCard(JACK_OF_SPADES)]
/// );
///
/// assert_eq!(
///   actions_for_card(JACK_OF_SPADES, Seven, Spades, &settings).collect::<Vec<_>>(),
///   vec![PlayCard(JACK_OF_SPADES)]
/// );
///
/// assert_eq!(
///   actions_for_card(JACK_OF_SPADES, Jack, Spades, &settings).collect::<Vec<_>>(),
///   vec![PlayCard(JACK_OF_SPADES)]
/// );
///
/// // You can always play a wild, and select the next suit
///
///  assert_eq!(
///      actions_for_card(EIGHT_OF_HEARTS, Two, Clubs, &settings).collect::<Vec<_>>(),
///      vec![
///          PlayWild(EIGHT_OF_HEARTS, Clubs),
///          PlayWild(EIGHT_OF_HEARTS, Diamonds),
///          PlayWild(EIGHT_OF_HEARTS, Hearts),
///          PlayWild(EIGHT_OF_HEARTS, Spades)
///      ]
///  );
///
/// ```
pub fn actions_for_card(
    card: Card,
    current_rank: Rank,
    current_suit: Suit,
    settings: &Settings,
) -> impl Iterator<Item = Action> {
    let is_wild: bool = settings.is_wild(card);
    let can_be_played_regularly: bool =
        !is_wild && (card.suit() == current_suit || card.rank() == current_rank);

    SUITS
        .iter()
        .map(move |&suit| PlayWild(card, suit))
        .filter(move |_| is_wild)
        .chain(can_be_played_regularly.then_some(PlayCard(card)))
}
