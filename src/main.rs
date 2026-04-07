use crate::{engine::prelude::*, players::{example_bot::ExampleBot, jack_papel_bots::hallucinator::Hallucinator}};

mod engine;
mod players;

fn main() {
    use players::human_controlled_bot::HumanControlledBot;

    let mut game: GameEngine<HumanControlledBot, ExampleBot> = GameEngine::new();

    game.run_game();
}