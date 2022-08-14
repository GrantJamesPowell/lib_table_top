use crate::{
    common::deck::{cards::QUEEN_OF_SPADES, Card, Rank, Suit},
    play::{number_of_players::FOUR_PLAYER, Player},
    utilities::PlayerIndexedData as PID,
};
use arrayvec::ArrayVec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trick {
    lead: (Player, Card),
    followed: ArrayVec<(Player, Card), 3>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Round(ArrayVec<Trick, 13>);

impl Trick {
    pub fn is_complete(&self) -> bool {
        self.followed.is_full()
    }

    pub fn played(&self) -> impl Iterator<Item = (Player, Card)> + '_ {
        Some(self.lead)
            .into_iter()
            .chain(self.followed.clone().into_iter())
    }

    pub fn winner(&self) -> Option<Player> {
        self.is_complete()
            .then(|| {
                self.played()
                    .filter(|(_player, card)| card.suit() == self.suit_lead())
                    .map(|(player, card)| (player, card.rank()))
                    .max_by(|(_, rank1), (_, rank2)| Rank::cmp_with_ace_high(rank1, rank2))
                    .map(|(player, _)| player)
            })
            .expect("we already check it's complete, so there should be at least one card")
    }

    pub fn suit_lead(&self) -> Suit {
        self.lead.1.suit()
    }

    pub fn points(&self) -> u8 {
        self.played()
            .map(|(_player, card)| {
                if card == QUEEN_OF_SPADES {
                    13
                } else if card.suit() == Suit::Hearts {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

impl Round {
    fn is_complete(&self) -> bool {
        self.0.is_full()
    }

    fn score(&self) -> PID<u8> {
        let points_awared = self
            .0
            .iter()
            .filter_map(|trick| trick.winner().map(|winner| (winner, trick.points())));

        let scores = points_awared.fold(
            FOUR_PLAYER.player_indexed_data(|_player| 0),
            |mut scores, (player, points)| {
                scores.get_mut(player).map(|score| *score + points);
                scores
            },
        );

        let moonshot = scores
            .iter()
            .filter(|&(_player, score)| *score == 26)
            .map(|(player, _score)| player)
            .next();

        match moonshot {
            None => scores,
            Some(shooter) => {
                FOUR_PLAYER.player_indexed_data(|player| if player == shooter { 0 } else { 26 })
            }
        }
    }
}
