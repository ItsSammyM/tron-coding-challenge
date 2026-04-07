use crate::engine::prelude::*;
use crate::players::{
    example_bot::ExampleBot,
    human_controlled_bot::HumanControlledBot
};

mod engine;
mod players;

fn main() {
    one_test_game::<ExampleBot, HumanControlledBot>();

    // run_game(vec![
    //     Player::new_player::<ExampleBot>(),
    //     Player::new_player::<HumanControlledBot>(),
    // ]);
}

fn one_test_game<O: Bot + 'static, X: Bot + 'static>(){
    GameEngine::new(BuildBot::<O>::new(), BuildBot::<X>::new()).run_game();
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