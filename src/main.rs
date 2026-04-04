use crate::engine::prelude::*;

mod engine;
mod players;

fn main() {
    use players::example_bot::ExampleBot;
    use players::bot_template::BotTemplate;
    let mut game: GameEngine<ExampleBot, BotTemplate> = GameEngine::new();
    game.run_game();
}
