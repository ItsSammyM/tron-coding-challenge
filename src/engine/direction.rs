/// The four directions a bot can move in.
/// Positive directions are up and to the right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// Up
    PositiveY,
    /// Down
    NegativeY,
    /// Right
    PositiveX,
    /// Left
    NegativeX,
}

impl Direction {
    /// Returns an iterator of all directions, in counterclockwise order starting with PositiveX.
    pub fn all() -> impl Iterator<Item = Direction> {
        Self::all_slice().into_iter()
    }

    /// Returns an array of all directions, in counterclockwise order starting with PositiveX.
    pub const fn all_slice() -> [Self; 4] {
        [
            Direction::PositiveX,
            Direction::PositiveY,
            Direction::NegativeX,
            Direction::NegativeY,
        ]
    }

    pub const fn up() -> Self {
        Direction::PositiveY
    }

    pub const fn down() -> Self {
        Direction::NegativeY
    }

    pub const fn left() -> Self {
        Direction::NegativeX
    }

    pub const fn right() -> Self {
        Direction::PositiveX
    }

    /// Returns the direction counterclockwise of this direction.
    pub const fn left_of(&self) -> Self{
        match self {
            Direction::PositiveY => Direction::NegativeX,
            Direction::NegativeY => Direction::PositiveX,
            Direction::PositiveX => Direction::PositiveY,
            Direction::NegativeX => Direction::NegativeY,
        }
    }

    /// Returns the direction clockwise of this direction.
    pub const fn right_of(&self) -> Self{
        match self {
            Direction::PositiveY => Direction::PositiveX,
            Direction::NegativeY => Direction::NegativeX,
            Direction::PositiveX => Direction::NegativeY,
            Direction::NegativeX => Direction::PositiveY,
        }
    }
}