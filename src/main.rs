#[cfg(all(feature = "competition", feature = "sample"))]
compile_error!("""Cannot have both the 'competition' and 'sample' features enabled at the same time. Please choose one or the other.""");

use regex::Regex;

use crate::engine::prelude::*;

#[cfg(feature = "competition")]
use crate::players::*;
#[cfg(feature = "competition")]
use competition::{Competition, CompetitionPlayer};

mod engine;
mod players;
mod competition;

/// To test your bot, set either OBot or XBot to your bot and run `cargo run`.
/// 
/// If you want to compare how well your bot does against another bot on average,
/// you can take a sample of 100 games using `cargo run --features=sample`.
/// 
/// If you want to run a full competition between every bot, run `cargo run --features=competition`.
/// Don't forget to add your bot to the list of competitors!
fn main() {
    use players::human_controlled_bot::HumanControlledBot;
    use players::example_bot::ExampleBot;

    type OBot = HumanControlledBot;
    type XBot = ExampleBot;

    Regex::new("").unwrap().replace("", "");

    #[cfg(all(not(feature = "sample"), not(feature = "competition")))]
    run_test_game_print::<OBot, XBot>();

    #[cfg(feature = "sample")]
    sample_games::<OBot, XBot>();

    #[cfg(feature = "competition")]
    Competition::run_and_print(vec![
        CompetitionPlayer::new_player::<example_bot::ExampleBot>(),
        CompetitionPlayer::new_player::<bot_template::BotTemplate>(),
        CompetitionPlayer::new_player::<stardustz_bots::StardustzBot>(),
        CompetitionPlayer::new_player::<jack_papel_bots::hallucinator::Hallucinator>(),
        CompetitionPlayer::new_player::<jack_papel_bots::rip_and_tear::RipAndTear>(),
        CompetitionPlayer::new_player::<jack_papel_bots::freedom_eater::FreedomEater>(),
        // Add your bot here!
    ]);
}

#[cfg(feature = "sample")]
fn sample_games<O: Bot + 'static, X: Bot + 'static>() {
    let mut o_games = 0;
    let mut draw_games = 0;
    let mut x_games = 0;

    let regex = Regex::new(r"([a-zA-Z0-9_]*::)*").unwrap();

    let o_name = regex.replace(std::any::type_name::<O>(), "");
    let x_name = regex.replace(std::any::type_name::<X>(), "");

    println!("Simulating 100 games between {} and {}...", o_name, x_name);

    for i in 0..100 {
        match run_test_game::<O, X>() {
            GameOver::Winner { player_who_won: PlayerId::O } => {
                println!("Round {}: {}", i + 1, o_name);
                o_games += 1;
            },
            GameOver::Winner { player_who_won: PlayerId::X } => {
                println!("Round {}: {}", i + 1, x_name);
                x_games += 1;
            },
            GameOver::Draw => {
                println!("Round {}: Draw", i + 1);
                draw_games += 1;
            },
        }
    }

    let total_games = o_games + draw_games + x_games;

    println!("\nRan 100 simulations: {}\n", total_games);
    println!("{}: {} ({:.2}%)", o_name, o_games, o_games as f64 / total_games as f64 * 100.0);
    println!("{}: {} ({:.2}%)", x_name, x_games, x_games as f64 / total_games as f64 * 100.0);
    println!("Draw: {} ({:.2}%)", draw_games, draw_games as f64 / total_games as f64 * 100.0);
}

#[cfg(feature = "sample")]
fn run_test_game<O: Bot + 'static, X: Bot + 'static>() -> GameOver{
    GameEngine::new(&BuildBot::<O>::new(), &BuildBot::<X>::new(), false).run_game()
}

#[cfg(not(feature = "sample"))]
fn run_test_game_print<O: Bot + 'static, X: Bot + 'static>(){
    GameEngine::new(
        &BuildBot::<O>::new(),
        &BuildBot::<X>::new(),
        true
    ).run_game_print();
}