use crate::{engine::{game_state, grid, prelude::*}, players::jack_papel_bots::{CellScore, JackBot, get_neighbors, pathfind}};

pub struct FreedomEater {
    my_player_id: PlayerId,
}

impl Bot for FreedomEater {
    fn new(my_player_id: PlayerId) -> Self {
        Self { my_player_id }
    }

    fn next_action(&mut self, game_state: &GameState) -> Direction {
        let grid = game_state.current_grid();
        let start = grid.player_head_position(self.my_player_id);

        let farthest_point = self.find_farthest_point(start, game_state);

        pathfind(start, farthest_point, grid)
            .and_then(|path| path.into_iter().nth(1))
            .map(|next_pos| self.direction_to(game_state, next_pos))
            .unwrap_or_else(|| {
                // If we can't find a path to the farthest point, just try to move in any direction that doesn't immediately crash us.
                self.ideal_directions(game_state).next()
                    .or_else(|| self.not_instant_crash_directions(game_state).next())
                    .unwrap_or(Direction::NegativeX)
            })
    }
}

impl FreedomEater {
    // This is a modified A* algorithm that finds the farthest reachable point from the current position.
    fn find_farthest_point(&self, start: GridPosition, game_state: &GameState) -> GridPosition {
        use std::collections::{BinaryHeap, HashMap};
        let grid = game_state.current_grid();

        // This is usually a max-heap, but CellScore has reverse ordering, so it's basically a min-heap.
        let mut open_set = BinaryHeap::new();

        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();

        g_score.insert(start, 0);
        open_set.push(CellScore(g_score[&start], start));
        let mut farthest = CellScore(0, start);

        while let Some(CellScore(score, current)) = open_set.pop() {
            if score > farthest.0 {
                farthest = CellScore(score, current);
            }

            for neighbor in get_neighbors(current, grid) {
                let tentative_g_score = g_score[&current] + 1;
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    if !open_set.iter().any(|&CellScore(_, pos)| pos == neighbor) {
                        open_set.push(CellScore(g_score[&neighbor], neighbor));
                    }
                }
            }
        }

        farthest.1
    }
}

impl JackBot for FreedomEater {
    fn my_player_id(&self) -> PlayerId {
        self.my_player_id
    }
}