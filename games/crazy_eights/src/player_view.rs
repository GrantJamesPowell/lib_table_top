use lttcore::common::deck::Card;
use lttcore::View;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Update {
    AddCards(Vec<Card>),
    RemoveCards(Vec<Card>),
}

use Update::*;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PlayerView {
    hand: Vec<Card>,
}

impl PlayerView {
    fn new(hand: Vec<Card>) -> Self {
        Self { hand }
    }

    fn hand(&self) -> &[Card] {
        &self.hand
    }
}

impl View for PlayerView {
    type Update = Update;

    fn update(&mut self, update: &Self::Update) -> Result<(), Box<(dyn Error)>> {
        match update {
            AddCards(cards) => {
                self.hand.extend_from_slice(cards);
            }
            RemoveCards(cards) => self.hand.retain(|card| !cards.contains(card)),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lttcore::common::deck::{cards::*, Card};

    #[test]
    fn test_you_can_add_cards() {
        let mut player_view: PlayerView = Default::default();
        assert!(player_view.hand().is_empty());

        let update = Update::AddCards(vec![ACE_OF_SPADES, TWO_OF_CLUBS, THREE_OF_DIAMONDS]);
        player_view
            .update(&update)
            .expect("you can update the hand");
        assert_eq!(
            player_view.hand(),
            [ACE_OF_SPADES, TWO_OF_CLUBS, THREE_OF_DIAMONDS]
        );

        let update = Update::RemoveCards(vec![ACE_OF_SPADES, THREE_OF_DIAMONDS]);
        player_view
            .update(&update)
            .expect("you can update the hand");
        assert_eq!(player_view.hand(), [TWO_OF_CLUBS]);
    }
}
