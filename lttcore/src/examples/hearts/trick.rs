use crate::{
    common::deck::{
        Card,
        Rank::{self, *},
        Suit::{self, *},
    },
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    PlayerAlreadyPlayed,
    TrickAlreadyFinished,
    CardAlreadyPlayed,
}

impl Trick {
    pub fn from_players_and_plays(plays: [(impl Into<Player>, impl Into<Card>); 4]) -> Self {
        let [x1, x2, x3, x4] = plays.map(|(p, c)| (p.into(), c.into()));
        Trick {
            lead: x1,
            followed: ArrayVec::from([x2, x3, x4]),
        }
    }

    pub fn from_lead(player: impl Into<Player>, card: impl Into<Card>) -> Self {
        Trick {
            lead: (player.into(), card.into()),
            followed: ArrayVec::new(),
        }
    }

    pub fn play(&mut self, player: impl Into<Player>, card: impl Into<Card>) -> Result<(), Error> {
        use Error::*;

        let player = player.into();
        let card = card.into();

        for (p, c) in self.played() {
            if p == player {
                return Err(PlayerAlreadyPlayed);
            }
            if c == card {
                return Err(CardAlreadyPlayed);
            }
        }

        self.followed
            .try_push((player, card))
            .map_err(|_| Error::TrickAlreadyFinished)
    }

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
            .map(|(_player, card)| match (card.suit(), card.rank()) {
                (Spades, Queen) => 13,
                (Hearts, _) => 1,
                (_, _) => 0,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::c;

    #[test]
    fn test_trick_complete() {
        let mut trick = Trick::from_lead(1, c!(k, s));
        assert!(!trick.is_complete());

        for (player, card) in [(2, c!(a, s)), (3, c!(10, s)), (4, c!(9, h))] {
            let result = trick.play(player, card);
            assert!(result.is_ok());
        }

        assert!(trick.is_complete());
    }

    #[test]
    fn test_suit_lead() {
        let trick = Trick::from_lead(1, c!(k, s));
        assert_eq!(trick.suit_lead(), Spades);
    }

    #[test]
    fn test_trick_points() {
        let mut trick = Trick::from_lead(1, c!(k, s));
        assert_eq!(trick.points(), 0);

        trick.play(2, c!(9, h)).expect("valid play");
        assert_eq!(trick.points(), 1);

        trick.play(3, c!(2, h)).expect("valid play");
        assert_eq!(trick.points(), 2);

        trick.play(4, c!(q, s)).expect("valid play");
        assert_eq!(trick.points(), 15);
    }

    #[test]
    fn test_winner_all_of_same_suit() {
        let trick = Trick::from_players_and_plays([
            (1, c!(8, s)),
            (2, c!(5, s)),
            (3, c!(7, s)),
            (4, c!(k, s)),
        ]);

        let expected_winner: Player = 4.into();
        let winner = trick.winner().expect("round is over");
        assert_eq!(winner, expected_winner);
    }

    #[test]
    fn test_winner_with_mixed_suits() {
        let trick = Trick::from_players_and_plays([
            (1, c!(8, s)),
            (2, c!(5, s)),
            (3, c!(7, h)),
            (4, c!(k, d)),
        ]);

        let expected_winner: Player = 1.into();
        let winner = trick.winner().expect("round is over");
        assert_eq!(winner, expected_winner);
    }
}
