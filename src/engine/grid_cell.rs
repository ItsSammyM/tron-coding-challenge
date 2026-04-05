use std::fmt::Display;

use crate::engine::prelude::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GridCell {
    Empty,
    Tail(PlayerId, Direction),
    Head(PlayerId, Direction),
}
impl GridCell {
    pub fn is_empty(&self) -> bool {
        *self == GridCell::Empty
    }
    pub fn is_not_empty(&self) -> bool {
        *self != GridCell::Empty
    }
    pub fn is_head(&self) -> bool {
        matches!(self, GridCell::Head(..))
    }
    pub fn is_tail(&self) -> bool {
        matches!(self, GridCell::Tail(..))
    }
    pub fn is_players_head(&self, player: PlayerId) -> bool {
        if let GridCell::Head(p, ..) = self {
            player == *p
        } else {
            false
        }
    }
}
impl Display for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridCell::Empty => " .",
                GridCell::Tail(player_id, _direction) if player_id.is_o() => " o",
                GridCell::Tail(_player_id, _direction) => " x",
                GridCell::Head(_player_id, direction) => match direction {
                    Direction::PositiveY => " ^",
                    Direction::NegativeY => " v",
                    Direction::PositiveX => " >",
                    Direction::NegativeX => " <",
                },
            }
        )
    }
}
