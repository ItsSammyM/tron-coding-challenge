use std::fmt::Display;

use crate::engine::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    grid_history: Vec<Grid>,
    game_over: Option<GameOver>,
}
impl GameState {
    pub fn new() -> Self {
        Self {
            grid_history: Vec::from([Grid::new_default()]),
            game_over: None,
        }
    }
    /// gets the grid of the most recently generated frame
    pub fn current_grid(&self) -> &Grid {
        self.grid_history
            .last()
            .expect("game state must have at least 1 grid")
    }
    /// starts at 0 and increases by 1 with each frame. This gets the time of the most recently generated frame
    pub fn current_time(&self) -> usize {
        self.grid_history
            .len()
            .checked_sub(1)
            .expect("game state must have at least 1 grid")
    }
    pub fn grid(&self, time_since_start: usize) -> Option<&Grid> {
        self.grid_history.get(time_since_start)
    }
    pub fn grid_history(&self) -> impl Iterator<Item = &Grid> {
        self.grid_history.iter()
    }
    pub fn is_game_over(&self) -> bool {
        self.game_over.is_some()
    }
    /// returns true if game not over
    pub fn go_to_next_frame(
        &mut self,
        player_a_choice: Direction,
        player_b_choice: Direction,
    ) -> NextFrameResult {
        if let Some(game_over) = self.game_over {
            return game_over.into()
        }

        let next_frame_result = self
            .current_grid()
            .next_grid(
                player_a_choice,
                player_b_choice,
                self.current_time() + 1
            );
        match &next_frame_result {
            NextFrameResult::NextFrame(grid) => {
                self.grid_history.push(grid.clone());
            }
            NextFrameResult::Winner { player_who_won } => {
                self.game_over = Some(GameOver::Winner { player_who_won: *player_who_won });
            }
            NextFrameResult::Draw => {
                self.game_over = Some(GameOver::Draw);
            }
        }
        next_frame_result
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.game_over {
            Some(game_over) => writeln!(f, "{}", game_over),
            None => writeln!(f, "{}", self.current_grid()),
        }
    }
}
