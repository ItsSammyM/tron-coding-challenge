use crate::{engine::prelude::*, players::stardustz_bots::cnnml_bot::{helper::*, learning::LearningAlgorithm, model::{Model, ModelOutput}, model_engine::ModelEngine}};



pub struct CnnmlBot{
    args: BotArgs,
    memory: Option<Vec<f32>>,
    model: Model,
}

const LEARN_NAME: &'static str = "test";

fn run_learning_with_saves(){
    let mut learning = LearningAlgorithm::load_or_new(LEARN_NAME);
    learning.run_and_save(LEARN_NAME);
}

impl Bot for CnnmlBot{
    fn new(args: BotArgs) -> Self {
        let model = if let Some(model) = load("model_saves/finalized") {model} else {Model::default()};
        CnnmlBot::new_from_model(model, args)
    }

    fn next_action(&mut self, game_state: &GameState) -> Direction {
        let engine = ModelEngine::new(
            self.args,
            game_state.current_grid(),
            &self.model
        );

        let ModelOutput{
            memory,
            direction
        } = engine.get_model_next_step(self.memory.take().unwrap());

        self.memory = Some(memory);
        engine.parse_model_output(direction)
    }
}

impl CnnmlBot{
    pub fn new_from_model(model: Model, args: BotArgs)->Self{
        CnnmlBot{
            args,
            memory: Some(vec![0.0; Model::MEMORY_SIZE]),
            model
        }
    }
}





