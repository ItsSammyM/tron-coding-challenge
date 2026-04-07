use regex::Regex;

use crate::engine::prelude::*;

mod engine;
mod players;

fn main() {
    use players::human_controlled_bot::HumanControlledBot;
    use players::example_bot::ExampleBot;

    type OBot = HumanControlledBot;
    type XBot = ExampleBot;

    Regex::new("").unwrap().replace("", "");

    let mut game: GameEngine<OBot, XBot> = GameEngine::new();

    #[cfg(not(feature = "sample_games"))]
    {
        game.run_game();
    }
    #[cfg(feature = "sample_games")]
    {
        sample_games::<OBot, XBot>();
    }
}

#[cfg(feature = "sample_games")]
fn sample_games<T: Bot, U: Bot>() {
    let mut o_games = 0;
    let mut draw_games = 0;
    let mut x_games = 0;
    let mut panicked_games = 0;

    let regex = Regex::new(r"([a-zA-Z0-9_]*::)*").unwrap();

    let o_name = regex.replace(std::any::type_name::<T>(), "");
    let x_name = regex.replace(std::any::type_name::<U>(), "");

    println!("Simulating 100 games between {} and {}...", o_name, x_name);

    for i in 0..100 {
        let mut game: GameEngine<T, U> = GameEngine::new();

        let Ok(result) = ({
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| game.run_game()))
        }) else {
            panicked_games += 1;
            println!("\nRound {}: Panicked!", i + 1);
            continue;
        };

        match result {
            GameOver::Winner { player_who_won } if player_who_won.is_o() => {
                println!("Round {}: {}", i + 1, o_name);
                o_games += 1;
            },
            GameOver::Winner { player_who_won } => {
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
    println!("\nPanicked (Not counted): {} ({:.2}% of all games)", panicked_games, panicked_games as f64 / (total_games as f64 + panicked_games as f64) * 100.0);
}