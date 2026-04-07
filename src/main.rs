use crate::{engine::prelude::*, players::{example_bot::ExampleBot, jack_papel_bots::{freedom_eater::FreedomEater, hallucinator::Hallucinator}}};

mod engine;
mod players;

fn main() {
    use players::human_controlled_bot::HumanControlledBot;

    let mut game: GameEngine<Hallucinator, FreedomEater> = GameEngine::new();

    game.run_game();
}