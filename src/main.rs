use crate::engine::{GameEngine, GameState};

mod engine;
mod players;

fn main() {
    use players::example_bot::ExampleBot;
    let mut game: GameEngine<ExampleBot, ExampleBot> = GameEngine::new();
    game.run_game();
}
