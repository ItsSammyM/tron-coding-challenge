use crate::engine::bot::BotFactory;
use crate::engine::prelude::*;
use crate::players::stardustz_bots::cnnml_bot::learning::ModelFactory;
use crate::players::stardustz_bots::cnnml_bot::model::Model;
use crate::players::{
    chatgpt_bots, example_bot, jack_papel_bots, stardustz_bots
};

pub fn opponents(model: &Model) -> Vec<Box<dyn BotFactory>> {
    vec![
        BuildBot::<example_bot::ExampleBot>::new_boxed(),
        BuildBot::<stardustz_bots::StardustzBot>::new_boxed(),
        Box::new(ModelFactory{model: model.clone()}),
        BuildBot::<jack_papel_bots::rip_and_tear::RipAndTear>::new_boxed(),
        BuildBot::<chatgpt_bots::myr::Myr>::new_boxed()
    ]
}