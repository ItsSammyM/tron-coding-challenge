use std::marker::PhantomData;

use crate::engine::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct BotArgs{
    debug: bool,
    player: PlayerId
}
impl BotArgs{
    pub fn new(player: PlayerId, debug: bool) -> Self {
        Self { debug, player }
    }
    pub fn my_player(&self) -> PlayerId {
        self.player
    }
    pub fn debug_mode(&self) -> bool {
        self.debug
    }
}

pub trait Bot {
    fn new(args: BotArgs) -> Self;
    fn next_action(&mut self, game_state: &GameState) -> Direction;
}

pub trait BotFactory {
    fn new_bot(&self, args: BotArgs) -> Box<dyn BotActionGenerator>;
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
    fn new_bot(&self, args: BotArgs) -> Box<dyn BotActionGenerator> {
        Box::new(B::new(args))
    }
}

impl<B: Bot> BotActionGenerator for B {
    fn generate_next_action(&mut self, game_state: &GameState) -> Direction {
        self.next_action(game_state)
    }
}