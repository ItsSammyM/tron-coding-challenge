use std::thread::JoinHandle;

use rand::{rng, rngs::ThreadRng};
use serde::{Deserialize, Serialize};

use crate::{
    GRID_SIZE,
    engine::prelude::*,
    players::stardustz_bots::{CnnmlBot, cnnml_bot::{helper::{load, save}, model::Model, opponents::opponents}}
};

const LEARN_NAME: &'static str = "test";

pub fn run_learning_with_saves(){
    let mut learning = LearningAlgorithm::load_or_new(LEARN_NAME);
    learning.run_and_save(LEARN_NAME);
}

#[derive(Serialize, Deserialize)]
pub struct LearningAlgorithm {
    current_best: Vec<(Model, f32)>,
}
impl LearningAlgorithm{
    const LEARNING_RATE_MAX: f32 = 2.0;
    const NUM_BEST_BOTS: usize = 15;
    pub fn new()->Self{
        Self { current_best: vec![(Model::default(), f32::NEG_INFINITY)] }
    }
    pub fn load_or_new(name: &str)->Self{
        if let Some(learning) = load(&format!("evolution_saves/{}", name)) {
            learning
        }else{
            Self::new()
        }
    }
    pub fn run_and_save(&mut self, name: &str) -> ! {
        loop{
            self.run_one_generation();
            save(self,&format!("evolution_saves/{}", name));
        }
    }
    fn run_one_generation(&mut self){

        let mut rng = rng();
        let mut new_models: Vec<Model> = Self::generate_new_models(
            self.current_best.drain(0..self.current_best.len()).into_iter().map(|(model,_)|model),
            &mut rng,
            Self::LEARNING_RATE_MAX
        );

        let mut join_handles: Vec<JoinHandle<(Model, f32)>> = Vec::new();
        while let Some(new_model) = new_models.pop() {
            join_handles.push(
                std::thread::spawn(move ||{
                    let other_bots = opponents(&new_model);
                    let score = Self::score(&new_model, &other_bots);
                    (new_model, score)
                })
            )
        }

        let mut scored_models: Vec<(Model, f32)> = join_handles.into_iter().map(|join|join.join().ok().unwrap()).collect();

        scored_models.sort_by(|(_, score_a), (_, score_b)|score_b.total_cmp(score_a));

        self.current_best = scored_models
        .into_iter()
        .take(Self::NUM_BEST_BOTS)
        .map(|(model, score)|{
            println!("new_best {}", score);
            (model,score)
        })
        .collect();
    }

    fn generate_new_models(models: impl Iterator<Item = Model>, rng: &mut ThreadRng, max_rate: f32) -> Vec<Model> {
        models
            .into_iter()
            .flat_map(|model|
                Self::generate_new_models_from_one(model, rng, max_rate).into_iter()
            )
            .collect()
    }
    fn generate_new_models_from_one(model: Model, rng: &mut ThreadRng, max_rate: f32) -> Vec<Model> {
        let mut out: Vec<Model> = (0..(200/Self::NUM_BEST_BOTS)).into_iter().map(|_|model.clone().randomize(rng, max_rate)).collect();
        out.push(model);
        out
    }

    fn score(model: &Model, other_bots: &Vec<Box<dyn BotFactory>>)->f32{
        let bots_score = other_bots
            .iter()
            .map(|other|{
                Self::score_against_bot(model, other.as_ref())
            })
            .fold(0.0, |a, b|a+b) / other_bots.len() as f32;
        println!("bots_score {}", bots_score);
        bots_score
    }

    fn score_against_bot(model: &Model, other: &dyn BotFactory) -> f32 {
        Self::one_games_score(model, PlayerId::O, other) +
        Self::one_games_score(model, PlayerId::X, other)
    }

    // in order of worst score to best score
    // losing fast
    // losing but u lived for a while
    // drawing fast
    // drawing but u lived for a while
    // winning but u lived for a while
    // winning fast
    fn one_games_score(model: &Model, me: PlayerId, other: &dyn BotFactory) -> f32 {

        const MAX_FRAMES_POSSIBLE: usize = GRID_SIZE * GRID_SIZE / 2;
        const SETTINGS: GameSettings = GameSettings{
            random_spawns: false,
            debug_mode: false
        };

        let my_factory: Box<dyn BotFactory> = Box::new(ModelFactory{model: model.clone()});

        let mut game_engine = match me {
            PlayerId::O => GameEngine::new(my_factory.as_ref(), other, SETTINGS),
            PlayerId::X => GameEngine::new(other, my_factory.as_ref(), SETTINGS),
        };

        let winner = game_engine.run_game();
        let num_frames = game_engine.game_state().current_time();

        match winner {
            GameOver::Winner { player_who_won } if player_who_won == me => 80f32 + 2f32*MAX_FRAMES_POSSIBLE as f32 - num_frames as f32,
            GameOver::Winner { .. } => num_frames as f32 - MAX_FRAMES_POSSIBLE as f32 - 80f32,
            GameOver::Draw => num_frames as f32,
        }
    }
}


pub struct ModelFactory{
    pub model: Model
}
impl BotFactory for ModelFactory{
    fn new_bot(&self, args: BotArgs) -> Box<dyn BotActionGenerator> {
        Box::new(CnnmlBot::new_from_model(self.model.clone(), args))
    }
}