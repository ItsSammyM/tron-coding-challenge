use crate::{engine::prelude::*, players::stardustz_bots::StardustzBot};

mod engine;
mod players;

fn main() {
    use players::hallucinator::Hallucinator;
    // use players::bot_template::BotTemplate;
    use players::stardustz_bots::ChaseBot;
    use players::stardustz_bots::SimpleSpaceFillBot;
    use players::human_controlled_bot::HumanControlledBot;

    let mut game: GameEngine<HumanControlledBot, StardustzBot> = GameEngine::new();

    game.run_game();
}