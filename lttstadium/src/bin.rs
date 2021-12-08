use lttcore::{
    bot::Contender,
    examples::{
        tic_tac_toe::{
            bot::prebuilt::{ExpertSkill, IntermediateSkill},
            TicTacToeBot,
        },
        TicTacToe,
    },
    play::Player,
};
use lttstadium::{FightCard, FightCardBuilder};

fn main() {
    let bots = vec![
        (Player::new(0), Contender::new(ExpertSkill.into_bot())),
        (Player::new(1), Contender::new(IntermediateSkill.into_bot())),
    ];

    let fight_card: FightCard<TicTacToe> = FightCardBuilder::default()
        .iterations(1_000_000_000)
        .contenders(bots.into_iter().collect())
        .build()
        .unwrap();

    fight_card.run(|(i, _)| {
        if i % 1000 == 0 {
            println!("Done {}", i)
        }
    })
}
