use lttcore::{
    bot::Contender,
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

fn main() {
    let bots = vec![
        (
            Player::new(0),
            Contender::new("Expert", TicTacToeBotWrapper(Expert)),
        ),
        (
            Player::new(1),
            Contender::new("Intermediate", TicTacToeBotWrapper(Intermediate)),
        ),
    ];

    let fight_card: FightCard<TicTacToe> = FightCardBuilder::default()
        .iterations(1_000_000_000)
        .contenders(bots.into_iter().collect())
        .build()
        .unwrap();

    fight_card.run(|(i, _, _)| {
        if i % 1000 == 0 {
            println!("Done {}", i)
        }
    })
}
