use std::fmt::Display;

use crate::engine::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerId{O, X}
impl PlayerId {
    pub fn new_o() -> PlayerId {
        PlayerId::O
    }
    pub fn new_x() -> PlayerId {
        PlayerId::X
    }
    pub fn is_o(&self) -> bool {
        *self == PlayerId::O
    }
    pub fn is_x(&self) -> bool {
        *self == PlayerId::X
    }
    pub fn other(&self) -> Self {
        match self {
            PlayerId::O => PlayerId::X,
            PlayerId::X => PlayerId::O,
        }
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
        write!(f, "{}", match self {
            PlayerId::O => "O",
            PlayerId::X => "X",
        })
    }
}
