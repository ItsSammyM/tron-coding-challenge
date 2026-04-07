use std::fmt::Display;

use crate::engine::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(bool);
impl PlayerId {
    pub fn new_o() -> PlayerId {
        PlayerId(true)
    }
    pub fn new_x() -> PlayerId {
        PlayerId(false)
    }
    pub fn is_o(&self) -> bool {
        self.0
    }
    pub fn is_x(&self) -> bool {
        !self.0
    }
    pub fn other(&self) -> Self {
        PlayerId(!self.0)
    }
    
    pub fn get_head_pos(&self, grid: &Grid)->GridPosition{
        grid.player_head_position(*self)
    }
    pub fn get_head_direction(&self, grid: &Grid)->Direction{
        grid.player_head_direction(*self)
    }
}
impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let player = match self.0 {
            true => "O",
            false => "X",
        };
        write!(f, "{}", player)
    }
}
