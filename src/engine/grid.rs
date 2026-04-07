use crate::engine::prelude::*;
use std::collections::HashMap;

pub const GRID_SIZE: usize = 21;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid([GridCell; GRID_SIZE * GRID_SIZE]);
impl Grid {
    pub fn new_default() -> Self {
        let mut out = Self([const { GridCell::Empty }; GRID_SIZE * GRID_SIZE]);
        *out.try_get_cell_mut((9, 10)).expect("pos is in bounds") =
            GridCell::Head(PlayerId::new_o(), Direction::NegativeX);
        *out.try_get_cell_mut((11, 10)).expect("pos is in bounds") =
            GridCell::Head(PlayerId::new_x(), Direction::PositiveX);
        out
    }
    pub fn next_grid(&self, player_a_choice: Direction, player_b_choice: Direction, next_frame: usize) -> NextFrameResult {

        //function is a hot mess

        let (a_pos, b_pos) = self.player_head_positions();

        let next_a_pos = a_pos.after_moved(player_a_choice);
        let next_b_pos = b_pos.after_moved(player_b_choice);

        if next_a_pos.is_none() && next_b_pos.is_none() {
            return NextFrameResult::Draw;
        };

        let Some(next_a_pos) = next_a_pos else {
            return NextFrameResult::Winner {
                player_who_won: PlayerId::new_x(),
            };
        };
        let Some(next_b_pos) = next_b_pos else {
            return NextFrameResult::Winner {
                player_who_won: PlayerId::new_o(),
            };
        };

        if next_a_pos == next_b_pos {
            return NextFrameResult::Draw;
        };

        let next_a_cell = self.get_cell(next_a_pos);
        let next_b_cell = self.get_cell(next_b_pos);

        let a_blocked = next_a_cell.is_not_empty();
        let b_blocked = next_b_cell.is_not_empty();

        if a_blocked && b_blocked {
            return NextFrameResult::Draw;
        };
        if a_blocked {
            return NextFrameResult::Winner {
                player_who_won: PlayerId::new_x(),
            };
        };
        if b_blocked {
            return NextFrameResult::Winner {
                player_who_won: PlayerId::new_o(),
            };
        };

        let mut out = self.clone();
        *out.get_cell_mut(a_pos) = GridCell::Tail(PlayerId::new_o(), player_a_choice, next_frame);
        *out.get_cell_mut(b_pos) = GridCell::Tail(PlayerId::new_x(), player_b_choice, next_frame);
        *out.get_cell_mut(next_a_pos) = GridCell::Head(PlayerId::new_o(), player_a_choice);
        *out.get_cell_mut(next_b_pos) = GridCell::Head(PlayerId::new_x(), player_b_choice);

        NextFrameResult::NextFrame(out)
    }

    pub fn get_cell_mut(&mut self, pos: impl Into<GridPosition>) -> &mut GridCell {
        self.0
            .get_mut(pos.into().i())
            .expect("position is in bounds")
    }
    pub fn try_get_cell_mut(&mut self, pos: impl TryInto<GridPosition>) -> Option<&mut GridCell> {
        self.0.get_mut(pos.try_into().ok()?.i())
    }
    pub fn get_cell(&self, pos: impl Into<GridPosition>) -> &GridCell {
        self.0
            .get(pos.into().i())
            .expect("position is in bounds")
    }
    pub fn try_get_cell(&self, pos: impl TryInto<GridPosition>) -> Option<&GridCell> {
        self.0.get(pos.try_into().ok()?.i())
    }

    pub fn head_positions_map(&self) -> HashMap<PlayerId, GridPosition> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(pos, cell)| {
                let GridCell::Head(player_id, ..) = cell else {
                    return None;
                };
                Some((
                    *player_id,
                    GridPosition::new_from_usize(pos).expect("position is valid"),
                ))
            })
            .collect()
    }
    /// reutrns (Player A Head Position, Player B Head Position)
    pub fn player_head_positions(&self) -> (GridPosition, GridPosition) {
        (
            self.player_head_position(PlayerId::new_o()),
            self.player_head_position(PlayerId::new_x()),
        )
    }
    /// reutrns (Player A Head Position, Player B Head Position)
    pub fn player_head_positions_slice(&self) -> [GridPosition; 2] {
        [
            self.player_head_position(PlayerId::new_o()),
            self.player_head_position(PlayerId::new_x()),
        ]
    }
    pub fn player_head_position(&self, id: PlayerId) -> GridPosition {
        self.0
            .iter()
            .enumerate()
            .find_map(|(pos, cell)| {
                let GridCell::Head(player_id, ..) = cell else {
                    return None;
                };
                if *player_id != id {
                    return None;
                };
                GridPosition::new_from_usize(pos)
            })
            .expect("both players must have a head")
    }
    pub fn player_head_direction(&self, id: PlayerId)->Direction{
        *self.0
            .iter()
            .enumerate()
            .find_map(|(_, cell)|{
                let GridCell::Head(player_id, direction) = cell else {return None};
                if *player_id != id {return None};
                Some(direction)
            })
            .expect("both players must have a head")
    }

    pub fn cell_is_empty(&self, pos: impl Into<GridPosition>) -> bool {
        self.get_cell(pos).is_empty()
    }
    pub fn cell_is_not_empty(&self, pos: impl Into<GridPosition>) -> bool {
        self.get_cell(pos).is_not_empty()
    }

    pub fn get_cells(&self) -> &[GridCell; GRID_SIZE * GRID_SIZE] {
        &self.0
    }
}
impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string: String = String::new();
        for row in (0..GRID_SIZE).rev() {
            for col in 0..GRID_SIZE {
                let cell = self.try_get_cell((col, row)).expect("in bounds");
                string += &format!("{}", cell);
            }
            string += &format!("\n");
        }
        write!(f, "{}", string)
    }
}
