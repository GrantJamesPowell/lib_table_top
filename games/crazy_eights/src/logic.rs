use crate::{
    Action::{self, *},
    Power::*,
    Settings,
};
use lttcore::common::deck::{Card, Rank, Suit, SUITS};

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
