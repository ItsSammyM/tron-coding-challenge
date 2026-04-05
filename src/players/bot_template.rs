use crate::engine::prelude::*;

pub struct BotTemplate {
    my_player_id: PlayerId,
}

impl Bot for BotTemplate {
    fn new(my_player_id: PlayerId) -> Self {
        BotTemplate { my_player_id }
    }

    fn next_action(&mut self, game_state: &GameState) -> Direction {
        Direction::PositiveX
    }
}
