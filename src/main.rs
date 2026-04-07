use crate::{engine::prelude::*, players::{example_bot::ExampleBot, jack_papel_bots::hallucinator::Hallucinator}};

mod engine;
mod players;

fn main() {
    use players::human_controlled_bot::HumanControlledBot;
    use players::stardustz_bots::StardustzBot;

    let mut game: GameEngine<StardustzBot, HumanControlledBot> = GameEngine::new();

    game.run_game();
}