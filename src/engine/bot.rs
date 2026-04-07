use std::marker::PhantomData;

use crate::engine::prelude::*;

pub trait Bot {
    fn new(my_player_id: PlayerId) -> Self;
    fn next_action(&mut self, game_state: &GameState) -> Direction;
}

pub trait BotFactory {
    fn new_bot(&self, my_player_id: PlayerId) -> Box<dyn BotActionGenerator>;
}

pub trait BotActionGenerator {
    fn generate_next_action(&mut self, game_state: &GameState) -> Direction;
}

pub struct BuildBot<B: Bot + 'static>{
    _marker: PhantomData<B>
}
impl<B: Bot + 'static> BuildBot<B> {
    pub fn new() -> Box<dyn BotFactory> {
        Box::new(Self{_marker: Default::default()})
    }
}

impl<B: Bot + 'static> BotFactory for BuildBot<B> {
    fn new_bot(&self, player_id: PlayerId) -> Box<dyn BotActionGenerator> {
        Box::new(B::new(player_id))
    }
}

impl<B: Bot> BotActionGenerator for B {
    fn generate_next_action(&mut self, game_state: &GameState) -> Direction {
        self.next_action(game_state)
    }
}