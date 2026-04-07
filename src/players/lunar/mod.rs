
#![expect(unused_imports)]
use core::hint::select_unpredictable;
use crate::engine::{direction, player_id, prelude::*};
const CELL_COUNT: usize = GRID_SIZE * GRID_SIZE;

mod pcg;
mod rngesus;
pub use rngesus::Rngesus;
mod game;
use game::*;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Lunar {
    id: PlayerId
}

impl Bot for Lunar {
    fn new(my_player_id: PlayerId) -> Self {
        Self {id: my_player_id}
    }
    fn next_action(&mut self, game_state: &GameState) -> Direction {
        todo!()
    }
}

const ALL_DIRECTIONS: [Direction; 4] = Direction::all_slice();

impl GridPosition {
    #[inline(always)]
    fn value(self) -> usize {
        // Safety: `GridPosition` is `repr(transparent)`
        unsafe { core::mem::transmute::<Self, usize>(self) }
    }
    /// # Safety
    /// Safe if `i` is in the grid.
    #[inline(always)]
    unsafe fn new_unchecked(i: usize) -> Self {
        debug_assert!(i < CELL_COUNT);
        // Safety: function precondition
        unsafe { core::hint::assert_unchecked(i < CELL_COUNT) };
        // Safety: `GridPosition` is `repr(transparent)`
        unsafe { core::mem::transmute::<usize, Self>(i) }
    }
}
impl Grid {
    // fn solo_move_through(self, player_id: PlayerId, pos_x: usize, pos_y: usize, d: Direction) -> Option<Self> {
    //     let direction = Vel::from(d);


    //     let blocked = next_pos.is_not_empty();

    //     if blocked {
    //         return None;
    //     };
    //     let cells = self.cells();
                
    // }

    fn cells(self) -> [[GridCell; GRID_SIZE]; GRID_SIZE] {
        // Safety: `Grid` is `repr(transparent)`.
        // Arrays containing arrays can be transmuted between 1 long array and themselves.
        unsafe {core::mem::transmute::<Self, [[GridCell; GRID_SIZE]; GRID_SIZE]>(self)}
    }
}