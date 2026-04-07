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

    #[cfg(not(feature = "sample_games"))]
    {
        one_test_game::<OBot, XBot>();
    }
    #[cfg(feature = "sample_games")]
    {
        sample_games::<OBot, XBot>();
    }
}

#[cfg(feature = "sample_games")]
fn sample_games<O: Bot + 'static, X: Bot + 'static>() {
    let mut o_games = 0;
    let mut draw_games = 0;
    let mut x_games = 0;

    let regex = Regex::new(r"([a-zA-Z0-9_]*::)*").unwrap();

    let o_name = regex.replace(std::any::type_name::<O>(), "");
    let x_name = regex.replace(std::any::type_name::<X>(), "");

    println!("Simulating 100 games between {} and {}...", o_name, x_name);

    for i in 0..100 {
        match one_test_game::<O, X>() {
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

fn one_test_game<O: Bot + 'static, X: Bot + 'static>() -> GameOver{
    GameEngine::new(BuildBot::<O>::new(), BuildBot::<X>::new()).run_game()
}

fn run_game(players: Vec<Player>){
    //code to make every player face eacother 6 times and track points
    todo!()
}

struct Player{
    name: String,
    points: f32,
    bot_factory: Box<dyn BotFactory>
}
impl Player{
    fn new_player<B: Bot + 'static>() -> Self {
        Self {
            name: "todo".to_string(),
            bot_factory: BuildBot::<B>::new(),
            points: 0.0
        }
    }
}