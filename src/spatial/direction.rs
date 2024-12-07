use super::Point2;

/// Represents an east, north, west or south direction.
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

    /// Iterate over the direction values in `Direction4`.
    pub fn values() -> Direction4Itr {
        Direction4Itr {
            next: Some(Direction4::East),
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

pub struct Direction4Itr {
    next: Option<Direction4>,
}

impl Iterator for Direction4Itr {
    type Item = Direction4;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;

        self.next = self.next.and_then(|d| match d {
            Direction4::East => Some(Direction4::North),
            Direction4::North => Some(Direction4::West),
            Direction4::West => Some(Direction4::South),
            Direction4::South => None,
        });

        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_point() {
        assert_eq!(
            <Direction4 as std::convert::Into<Point2>>::into(Direction4::East),
            Point2::new(1, 0)
        );
        assert_eq!(
            <Direction4 as std::convert::Into<Point2>>::into(Direction4::North),
            Point2::new(0, -1)
        );
        assert_eq!(
            <Direction4 as std::convert::Into<Point2>>::into(Direction4::West),
            Point2::new(-1, 0)
        );
        assert_eq!(
            <Direction4 as std::convert::Into<Point2>>::into(Direction4::South),
            Point2::new(0, 1)
        );
    }

    #[test]
    fn iterate_direction4() {
        assert_eq!(
            Direction4::values().collect::<Vec<Direction4>>(),
            [
                Direction4::East,
                Direction4::North,
                Direction4::West,
                Direction4::South
            ]
        );
    }
}
