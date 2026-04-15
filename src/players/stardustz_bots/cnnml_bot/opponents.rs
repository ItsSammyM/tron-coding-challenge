use crate::engine::bot::BotFactory;
use crate::engine::prelude::*;
use crate::players::stardustz_bots::cnnml_bot::learning::ModelFactory;
use crate::players::stardustz_bots::cnnml_bot::model::Model;
use crate::players::{
    example_bot,
    jack_papel_bots,
    stardustz_bots,
};

pub fn opponents(model: &Model) -> Vec<Box<dyn BotFactory>> {
    vec![
        BuildBot::<example_bot::ExampleBot>::new_boxed(),
        BuildBot::<stardustz_bots::StardustzBot>::new_boxed(),
        Box::new(ModelFactory{model: model.clone()}),
        BuildBot::<stardustz_bots::ChaseBot>::new_boxed(),
        BuildBot::<jack_papel_bots::hallucinator::Hallucinator>::new_boxed(),
        BuildBot::<jack_papel_bots::rip_and_tear::RipAndTear>::new_boxed(),
        BuildBot::<jack_papel_bots::freedom_eater::FreedomEater>::new_boxed(),

        BuildBot::<NegYBot>::new_boxed(),
        BuildBot::<PosYBot>::new_boxed(),
    ]
}

struct NegYBot;
impl Bot for NegYBot{
    fn new(_args: BotArgs) -> Self {
        Self
    }
    fn next_action(&mut self, _game_state: &GameState) -> Direction {
        Direction::NegativeY
    }
}
struct PosYBot;
impl Bot for PosYBot{
    fn new(_args: BotArgs) -> Self {
        Self
    }
    fn next_action(&mut self, _game_state: &GameState) -> Direction {
        Direction::PositiveY
    }
}