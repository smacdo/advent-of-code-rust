use super::Point2;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Direction4 {
    East,
    North,
    West,
    South,
}

impl Direction4 {
    /// Rotate clockwise, or "right" from the perspective of a North facing
    /// direction.
    pub fn rotated_90_cw(&self) -> Self {
        match self {
            Direction4::East => Direction4::South,
            Direction4::North => Direction4::East,
            Direction4::West => Direction4::North,
            Direction4::South => Direction4::West,
        }
    }
}

impl From<Direction4> for Point2 {
    fn from(value: Direction4) -> Self {
        match value {
            Direction4::East => Point2::new(1, 0),
            Direction4::North => Point2::new(0, -1),
            Direction4::West => Point2::new(-1, 0),
            Direction4::South => Point2::new(0, 1),
        }
    }
}
