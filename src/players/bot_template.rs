use crate::engine::prelude::*;

pub struct BotTemplate {
    _my_player_id: PlayerId,
}

impl Bot for BotTemplate {
    fn new(args: BotArgs) -> Self {
        BotTemplate { _my_player_id: args.my_player() }
    }

    fn next_action(&mut self, _game_state: &GameState) -> Direction {
        Direction::PositiveX
    }
}
