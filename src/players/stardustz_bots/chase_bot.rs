use crate::{engine::prelude::*, players::stardustz_bots::{a_star::Astar, helper::{DirectionIterator, players_only_not_crash_direction}}};

pub struct ChaseBot{
    my_player_id: PlayerId,
}

impl Bot for ChaseBot{
    fn new(my_player_id: PlayerId)->Self {
        ChaseBot{my_player_id}
    }

    fn next_action(&mut self, game_state: &GameState) -> Direction {
        let grid = game_state.current_grid();
        let my_pos = self.my_player_id.get_head_pos(grid);
        let enemy_pos = self.my_player_id.other().get_head_pos(grid);

        let Some(agro_direction) = Astar::a_star_direction(grid, my_pos, enemy_pos) else {
            return Direction::all().filter_not_crash(self.my_player_id, grid).next().unwrap_or(NegativeX)
        };

        if
            my_pos.after_moved(agro_direction) == players_only_not_crash_direction(self.my_player_id.other(), grid) &&
            let Some(dir) = Direction::all().filter_not_crash_into_head(self.my_player_id, grid).next()
        {
            return dir;
        }

        agro_direction
    }
}
