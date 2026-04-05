use crate::engine::prelude::*;

pub trait Bot {
    fn new(my_player_id: PlayerId) -> Self;
    fn next_action(&mut self, game_state: &GameState) -> Direction;
}
