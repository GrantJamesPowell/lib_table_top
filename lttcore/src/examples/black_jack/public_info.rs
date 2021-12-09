use crate::common::deck::Card;
use crate::play::{Player, Score, TurnNum, View};
use crate::utilities::PlayerIndexedData as PID;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Bet,
    PlayHand(Player, usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicInfo {
    pub statuses: PID<PlayerStatus>,
    pub phase: Phase,
    pub hands: PID<SmallVec<[Hand; 1]>>,
}

impl PublicInfo {
    pub fn dealer_card_showing(&self) -> Option<Card> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hand {
    pub bet: u32,
    pub cards: SmallVec<[Card; 4]>,
}

impl Hand {
    pub fn has_taken_additional_cards(&self) -> bool {
        todo!()
    }

    pub fn is_able_to_split(&self) -> bool {
        todo!()
    }

    pub fn is_able_to_double_down(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublicInfoUpdate {
    EndHand {
        dealer_hand: Hand,
        status_updates: PID<PlayerStatus>,
    },
    AddHand(Player, Hand),
    UpdateHand(Player, usize, Hand),
    UpdateStatus(PID<PlayerStatus>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayerStatus {
    Resigned { turn: TurnNum, chips: u32 },
    InPlay { chips: u32 },
    Busted { turn: TurnNum },
}

impl Score for PublicInfo {
    fn score(&self) -> Option<PID<i64>> {
        use PlayerStatus::*;

        Some(self.statuses.map(|status| match status {
            Resigned { chips, .. } | InPlay { chips, .. } => *chips as i64,
            Busted { .. } => 0,
        }))
    }
}

impl View for PublicInfo {
    type Update = PublicInfoUpdate;

    fn update(&mut self, update: Cow<'_, Self::Update>) {
        use PublicInfoUpdate::*;

        match update.into_owned() {
            EndHand {
                dealer_hand: _,
                status_updates,
            } => {
                self.statuses.extend(status_updates);
            }
            AddHand(player, hand) => {
                self.hands[player].push(hand);
            }
            UpdateHand(player, idx, hand) => {
                self.hands[player][idx] = hand;
            }
            UpdateStatus(status_updates) => {
                self.statuses.extend(status_updates);
            }
        }
    }
}
