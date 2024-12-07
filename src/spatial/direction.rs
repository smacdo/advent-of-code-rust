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

    /// Return an iterator over the directions in `Direction4`.
    pub fn itr() -> Direction4Itr {
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

/// Iterator over the enumeration values for `Direction4`.
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

/// Represents an east, north, west, south directions and the four diagonals
/// between.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Direction8 {
    East,
    Northeast,
    North,
    Northwest,
    West,
    Southwest,
    South,
    Southeast,
}

impl Direction8 {
    /// Return an iterator over the directions in `Direction8`.
    pub fn itr() -> Direction8Itr {
        Direction8Itr {
            next: Some(Direction8::East),
        }
    }
}

impl From<Direction8> for Point2 {
    fn from(value: Direction8) -> Self {
        match value {
            Direction8::East => Point2::new(1, 0),
            Direction8::Northeast => Point2::new(1, -1),
            Direction8::North => Point2::new(0, -1),
            Direction8::Northwest => Point2::new(-1, -1),
            Direction8::West => Point2::new(-1, 0),
            Direction8::Southwest => Point2::new(-1, 1),
            Direction8::South => Point2::new(0, 1),
            Direction8::Southeast => Point2::new(1, 1),
        }
    }
}

/// Iterator over the enumeration values for `Direction8`.
pub struct Direction8Itr {
    next: Option<Direction8>,
}

impl Iterator for Direction8Itr {
    type Item = Direction8;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next;

        self.next = self.next.and_then(|d| match d {
            Direction8::East => Some(Direction8::Northeast),
            Direction8::Northeast => Some(Direction8::North),
            Direction8::North => Some(Direction8::Northwest),
            Direction8::Northwest => Some(Direction8::West),
            Direction8::West => Some(Direction8::Southwest),
            Direction8::Southwest => Some(Direction8::South),
            Direction8::South => Some(Direction8::Southeast),
            Direction8::Southeast => None,
        });

        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction4_to_point() {
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
            Direction4::itr().collect::<Vec<Direction4>>(),
            [
                Direction4::East,
                Direction4::North,
                Direction4::West,
                Direction4::South
            ]
        );
    }

    #[test]
    fn direction8_to_point() {
        assert_eq!(
            <Direction8 as std::convert::Into<Point2>>::into(Direction8::East),
            Point2::new(1, 0)
        );
        assert_eq!(
            <Direction8 as std::convert::Into<Point2>>::into(Direction8::Northeast),
            Point2::new(1, -1)
        );
        assert_eq!(
            <Direction8 as std::convert::Into<Point2>>::into(Direction8::North),
            Point2::new(0, -1)
        );
        assert_eq!(
            <Direction8 as std::convert::Into<Point2>>::into(Direction8::Northwest),
            Point2::new(-1, -1)
        );
        assert_eq!(
            <Direction8 as std::convert::Into<Point2>>::into(Direction8::West),
            Point2::new(-1, 0)
        );
        assert_eq!(
            <Direction8 as std::convert::Into<Point2>>::into(Direction8::Southwest),
            Point2::new(-1, 1)
        );
        assert_eq!(
            <Direction8 as std::convert::Into<Point2>>::into(Direction8::South),
            Point2::new(0, 1)
        );
        assert_eq!(
            <Direction8 as std::convert::Into<Point2>>::into(Direction8::Southeast),
            Point2::new(1, 1)
        );
    }

    #[test]
    fn iterate_direction8() {
        assert_eq!(
            Direction8::itr().collect::<Vec<Direction8>>(),
            [
                Direction8::East,
                Direction8::Northeast,
                Direction8::North,
                Direction8::Northwest,
                Direction8::West,
                Direction8::Southwest,
                Direction8::South,
                Direction8::Southeast,
            ]
        );
    }
}
