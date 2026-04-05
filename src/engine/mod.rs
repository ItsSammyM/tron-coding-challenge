use crate::engine::prelude::*;
use std::fmt::Display;

pub mod prelude;

pub mod bot;
pub mod direction;
pub mod game_engine;
pub mod game_state;
pub mod grid;
pub mod grid_cell;
pub mod grid_position;
pub mod player_id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameOver {
    Winner { player_who_won: PlayerId },
    Draw,
}
impl Display for GameOver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameOver::Winner { player_who_won } =>
                    format!("Game over: Player {} won", player_who_won),
                GameOver::Draw => "Game over: Draw".to_owned(),
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextFrameResult {
    NextFrame(Grid),
    Winner { player_who_won: PlayerId },
    Draw,
}
