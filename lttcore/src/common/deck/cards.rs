use crate::common::deck::{Card, Rank::*, Suit::*};

pub const ACE_OF_SPADES: Card = Card::new(Ace, Spades);
pub const ACE_OF_HEARTS: Card = Card::new(Ace, Hearts);
pub const ACE_OF_CLUBS: Card = Card::new(Ace, Clubs);
pub const ACE_OF_DIAMONDS: Card = Card::new(Ace, Diamonds);

pub const KING_OF_SPADES: Card = Card::new(King, Spades);
pub const KING_OF_HEARTS: Card = Card::new(King, Hearts);
pub const KING_OF_CLUBS: Card = Card::new(King, Clubs);
pub const KING_OF_DIAMONDS: Card = Card::new(King, Diamonds);

pub const QUEEN_OF_SPADES: Card = Card::new(Queen, Spades);
pub const QUEEN_OF_HEARTS: Card = Card::new(Queen, Hearts);
pub const QUEEN_OF_CLUBS: Card = Card::new(Queen, Clubs);
pub const QUEEN_OF_DIAMONDS: Card = Card::new(Queen, Diamonds);

pub const JACK_OF_SPADES: Card = Card::new(Jack, Spades);
pub const JACK_OF_HEARTS: Card = Card::new(Jack, Hearts);
pub const JACK_OF_CLUBS: Card = Card::new(Jack, Clubs);
pub const JACK_OF_DIAMONDS: Card = Card::new(Jack, Diamonds);

pub const TEN_OF_SPADES: Card = Card::new(Ten, Spades);
pub const TEN_OF_HEARTS: Card = Card::new(Ten, Hearts);
pub const TEN_OF_CLUBS: Card = Card::new(Ten, Clubs);
pub const TEN_OF_DIAMONDS: Card = Card::new(Ten, Diamonds);

pub const NINE_OF_SPADES: Card = Card::new(Nine, Spades);
pub const NINE_OF_HEARTS: Card = Card::new(Nine, Hearts);
pub const NINE_OF_CLUBS: Card = Card::new(Nine, Clubs);
pub const NINE_OF_DIAMONDS: Card = Card::new(Nine, Diamonds);

pub const EIGHT_OF_SPADES: Card = Card::new(Eight, Spades);
pub const EIGHT_OF_HEARTS: Card = Card::new(Eight, Hearts);
pub const EIGHT_OF_CLUBS: Card = Card::new(Eight, Clubs);
pub const EIGHT_OF_DIAMONDS: Card = Card::new(Eight, Diamonds);

pub const SEVEN_OF_SPADES: Card = Card::new(Seven, Spades);
pub const SEVEN_OF_HEARTS: Card = Card::new(Seven, Hearts);
pub const SEVEN_OF_CLUBS: Card = Card::new(Seven, Clubs);
pub const SEVEN_OF_DIAMONDS: Card = Card::new(Seven, Diamonds);

pub const SIX_OF_SPADES: Card = Card::new(Six, Spades);
pub const SIX_OF_HEARTS: Card = Card::new(Six, Hearts);
pub const SIX_OF_CLUBS: Card = Card::new(Six, Clubs);
pub const SIX_OF_DIAMONDS: Card = Card::new(Six, Diamonds);

pub const FIVE_OF_SPADES: Card = Card::new(Five, Spades);
pub const FIVE_OF_HEARTS: Card = Card::new(Five, Hearts);
pub const FIVE_OF_CLUBS: Card = Card::new(Five, Clubs);
pub const FIVE_OF_DIAMONDS: Card = Card::new(Five, Diamonds);

pub const FOUR_OF_SPADES: Card = Card::new(Four, Spades);
pub const FOUR_OF_HEARTS: Card = Card::new(Four, Hearts);
pub const FOUR_OF_CLUBS: Card = Card::new(Four, Clubs);
pub const FOUR_OF_DIAMONDS: Card = Card::new(Four, Diamonds);

pub const THREE_OF_SPADES: Card = Card::new(Three, Spades);
pub const THREE_OF_HEARTS: Card = Card::new(Three, Hearts);
pub const THREE_OF_CLUBS: Card = Card::new(Three, Clubs);
pub const THREE_OF_DIAMONDS: Card = Card::new(Three, Diamonds);

pub const TWO_OF_SPADES: Card = Card::new(Two, Spades);
pub const TWO_OF_HEARTS: Card = Card::new(Two, Hearts);
pub const TWO_OF_CLUBS: Card = Card::new(Two, Clubs);
pub const TWO_OF_DIAMONDS: Card = Card::new(Two, Diamonds);

pub const ACES: [Card; 4] = [ACE_OF_CLUBS, ACE_OF_DIAMONDS, ACE_OF_HEARTS, ACE_OF_SPADES];
pub const KINGS: [Card; 4] = [
    KING_OF_CLUBS,
    KING_OF_DIAMONDS,
    KING_OF_HEARTS,
    KING_OF_SPADES,
];
pub const QUEENS: [Card; 4] = [
    QUEEN_OF_CLUBS,
    QUEEN_OF_DIAMONDS,
    QUEEN_OF_HEARTS,
    QUEEN_OF_SPADES,
];
pub const JACKS: [Card; 4] = [
    JACK_OF_CLUBS,
    JACK_OF_DIAMONDS,
    JACK_OF_HEARTS,
    JACK_OF_SPADES,
];
pub const TENS: [Card; 4] = [TEN_OF_CLUBS, TEN_OF_DIAMONDS, TEN_OF_HEARTS, TEN_OF_SPADES];
pub const NINES: [Card; 4] = [
    NINE_OF_CLUBS,
    NINE_OF_DIAMONDS,
    NINE_OF_HEARTS,
    NINE_OF_SPADES,
];
pub const EIGHTS: [Card; 4] = [
    EIGHT_OF_CLUBS,
    EIGHT_OF_DIAMONDS,
    EIGHT_OF_HEARTS,
    EIGHT_OF_SPADES,
];
pub const SEVENS: [Card; 4] = [
    SEVEN_OF_CLUBS,
    SEVEN_OF_DIAMONDS,
    SEVEN_OF_HEARTS,
    SEVEN_OF_SPADES,
];
pub const SIXS: [Card; 4] = [SIX_OF_CLUBS, SIX_OF_DIAMONDS, SIX_OF_HEARTS, SIX_OF_SPADES];
pub const FIVES: [Card; 4] = [
    FIVE_OF_CLUBS,
    FIVE_OF_DIAMONDS,
    FIVE_OF_HEARTS,
    FIVE_OF_SPADES,
];
pub const FOURS: [Card; 4] = [
    FOUR_OF_CLUBS,
    FOUR_OF_DIAMONDS,
    FOUR_OF_HEARTS,
    FOUR_OF_SPADES,
];
pub const THREES: [Card; 4] = [
    THREE_OF_CLUBS,
    THREE_OF_DIAMONDS,
    THREE_OF_HEARTS,
    THREE_OF_SPADES,
];
pub const TWOS: [Card; 4] = [TWO_OF_CLUBS, TWO_OF_DIAMONDS, TWO_OF_HEARTS, TWO_OF_SPADES];
