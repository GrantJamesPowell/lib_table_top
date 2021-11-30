use lttcore::{
    bot::Bot,
    examples::{
        tic_tac_toe::{
            bot::prebuilt::{Expert, Intermediate},
            bot::TicTacToeBotWrapper,
        },
        TicTacToe,
    },
    play::Player,
};
use lttstadium::{FightCard, FightCardBuilder};
use std::sync::Arc;

fn main() {
    let bots: Vec<(Player, Arc<dyn Bot<Game = TicTacToe>>)> = vec![
        (Player::new(0), Arc::new(TicTacToeBotWrapper(Expert))),
        (Player::new(1), Arc::new(TicTacToeBotWrapper(Intermediate))),
    ];

    let fight_card: FightCard<TicTacToe> = FightCardBuilder::default()
        .iterations(1_000_000)
        .bots(bots.into_iter().collect())
        .build()
        .unwrap();

    fight_card.run(|_| println!("here!"))
}
