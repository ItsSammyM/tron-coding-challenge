use crate::engine::prelude::*;
use crate::players::{
    example_bot::ExampleBot,
    human_controlled_bot::HumanControlledBot
};

mod engine;
mod players;

fn main() {
    one_game(
        Player::new_player(BuildBot::<ExampleBot>::new()),
        Player::new_player(BuildBot::<HumanControlledBot>::new())
    );

    // let players = vec![
    //     Player::new_player(BuildBot::<ExampleBot>::new()),
    //     Player::new_player(BuildBot::<HumanControlledBot>::new()),
    // ];

    // run_game(players);
}

fn one_game(a: Player, b: Player){
    GameEngine::new(a.bot_factory, b.bot_factory).run_game();
}

fn run_game(players: Vec<Player>){
    //code to make every player face eacother 6 times and track points
}

struct Player{
    name: String,
    points: f32,
    bot_factory: Box<dyn BotFactory>
}
impl Player{
    fn new_player(bot_factory: Box<dyn BotFactory>) -> Self {
        Self {
            name: "todo".to_string(),
            bot_factory,
            points: 0.0
        }
    }
}