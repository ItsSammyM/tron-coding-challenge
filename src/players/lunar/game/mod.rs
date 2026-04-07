use crate::engine::prelude::*;
use core::hint::select_unpredictable;
use core::mem;


#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Tail,
    HeadO,
    HeadX
}

impl From<GridCell> for Cell {
    fn from(value: GridCell) -> Self {
        match value {
            GridCell::Empty => Cell::Empty,
            GridCell::Head(id, _) => if id.is_o() {Cell::HeadO} else {Cell::HeadX},
            GridCell::Tail(..) => Cell::Tail
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Pos(pub usize, pub usize);

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Plane {
    pub grid: [[bool; GRID_SIZE]; GRID_SIZE], 
    pub x: Pos,
    pub o: Pos
}

impl From<Grid> for Plane {
    fn from(value: Grid) -> Self {
        let mut x_pos = Pos(0,0);
        let mut o_pos = Pos(0,0);
        let mut plane = [[false; GRID_SIZE]; GRID_SIZE];
        let cells = value.cells();
        for x in 0..=GRID_SIZE {
            for y in 0..=GRID_SIZE {
                let pos = Pos(x, y);
                let cell = cells[x][y];
                plane[x][y] = cell != GridCell::Empty;
                let is_x_head = match cell {
                    GridCell::Head(id, _) => id.is_x(),
                    _ => false
                };
                let is_o_head = match cell {
                    GridCell::Head(id, _) => id.is_o(),
                    _ => false
                };
                x_pos = select_unpredictable(
                    is_x_head,
                    pos.clone(),
                    x_pos
                );
                o_pos = select_unpredictable(
                    is_o_head,
                    pos,
                    o_pos
                );
            }
        }
        Self {
            grid: plane,
            x: x_pos,
            o: o_pos
        }
    }
}


#[repr(C)]
#[derive(Debug, Clone, Hash)]
#[cfg_attr(debug_assertions, derive(PartialEq, Eq))]
pub struct Vel(pub isize, pub isize);
impl Vel {
    pub const UP: Self = Self(0, 1);
    pub const DOWN: Self = Self(0, -1);
    pub const RIGHT: Self = Self(1, 0);
    pub const LEFT: Self = Self(-1, 0);

    pub const ALL: [Self; 4] = [Self::UP, Self::DOWN, Self::LEFT, Self::RIGHT];
}

impl From<Direction> for Vel {
    #[inline(always)]
    fn from(value: Direction) -> Self {
        let neg_x = value == Direction::NegativeX;
        let neg = neg_x | (value == Direction::NegativeY);
        let vel_1d = select_unpredictable(neg, -1, 1);
        let is_x = neg_x | (value == Direction::PositiveX);
        let ans = select_unpredictable(
            is_x,
            Vel(vel_1d, 0), 
            Vel(0, vel_1d)
        );
        if cfg!(debug_assertions) {
            let slow_ans = match value {
                Direction::NegativeX => Vel::LEFT,
                Direction::PositiveX => Vel::RIGHT,
                Direction::NegativeY => Vel::DOWN,
                Direction::PositiveY => Vel::UP
            };
            assert_eq!(slow_ans, ans)
        }
        ans
    }
}
impl From<Vel> for Direction {
    #[inline(always)]
    fn from(value: Vel) -> Self {
        let is_pos = (value.0 | value.1) == 1;
        let y = select_unpredictable(is_pos, Direction::PositiveY, Direction::NegativeY);
        let x = select_unpredictable(is_pos, Direction::PositiveX, Direction::NegativeY);
        let is_y = value.0 == 0;
        let ans = select_unpredictable(is_y, y, x);
        if cfg!(debug_assertions) {
            let slow_ans = match value {
                Vel::LEFT => Direction::NegativeX,
                Vel::RIGHT => Direction::PositiveX,
                Vel::DOWN => Direction::NegativeY,
                Vel::UP => Direction::PositiveY,
                _ => panic!("invalid velocity: {value:?}")
            };
            assert_eq!(slow_ans, ans)
        }
        ans
    }
}


pub enum GameEnd {
    Tie,
    WinO,
    WinX
}

impl Plane {
    #[inline]
    fn step(mut self, x_vel: Vel, x_pos: Pos, o_vel: Vel, o_pos: Pos) -> Result<Self, GameEnd> {
        let (x_wish_x, mut x_crash) = x_pos.0.overflowing_add_signed(x_vel.0);
        x_crash |= x_wish_x >= GRID_SIZE;
        let (x_wish_y, x_crash_y) = x_pos.1.overflowing_add_signed(x_vel.1);
        x_crash |= x_crash_y;
        x_crash |= x_wish_y >= GRID_SIZE;
        let x_wish = Pos(x_wish_x, x_wish_y);

        let (o_wish_x, mut o_crash) = x_pos.0.overflowing_add_signed(x_vel.0);
        o_crash |= o_wish_x >= GRID_SIZE;
        let (o_wish_y, o_crash_y) = x_pos.1.overflowing_add_signed(x_vel.1);
        o_crash |= o_crash_y;
        o_crash |= o_wish_y >= GRID_SIZE;
        let o_wish = Pos(o_wish_x, o_wish_y);

        let head_to_head = (x_wish_x == o_wish_x) & (x_wish_y == o_wish_y);

        // MUST BE `||`. Short circuit prevents oob.
        let x_crash = x_crash || mem::replace(&mut self.grid[x_wish_x][x_wish_y], true);
        let o_crash = o_crash || mem::replace(&mut self.grid[o_wish_x][o_wish_y], true);
        if (x_crash | o_crash) | head_to_head {
            return Err(
                if (x_crash & o_crash) | head_to_head {
                    GameEnd::Tie
                } else if x_crash {
                    GameEnd::WinO
                } else {
                    GameEnd::WinX
                }
            )
        }
        self.x = x_wish;
        self.o = o_wish;
        Ok(self)
    }
}