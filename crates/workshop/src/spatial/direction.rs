use std::str::FromStr;

use thiserror::Error;

use super::Point2;

const EAST_NAME: &str = "East";
const NORTHEAST_NAME: &str = "Northeast";
const NORTH_NAME: &str = "North";
const NORTHWEST_NAME: &str = "Northwest";
const WEST_NAME: &str = "West";
const SOUTHWEST_NAME: &str = "Southwest";
const SOUTH_NAME: &str = "South";
const SOUTHEAST_NAME: &str = "Southeast";

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
    ///
    /// ```
    /// use workshop::spatial::Direction4;
    ///
    /// assert_eq!(Direction4::North.rotated_90_cw(), Direction4::East);
    /// assert_eq!(Direction4::South.rotated_90_cw(), Direction4::West);
    /// ```
    pub fn rotated_90_cw(&self) -> Self {
        match self {
            Direction4::East => Direction4::South,
            Direction4::North => Direction4::East,
            Direction4::West => Direction4::North,
            Direction4::South => Direction4::West,
        }
    }

    /// Return an iterator over directions clockwise starting from East.
    ///
    /// ```
    /// use workshop::spatial::Direction4;
    ///
    /// for dir in Direction4::all() {
    ///   println!("{dir}");
    /// }
    ///
    /// assert_eq!(
    ///   Direction4::all().collect::<Vec<Direction4>>(),
    ///   [
    ///     Direction4::East,
    ///     Direction4::North,
    ///     Direction4::West,
    ///     Direction4::South,
    ///   ]
    /// );
    /// ```
    pub fn all() -> Direction4Itr {
        Direction4Itr {
            next: Some(Direction4::East),
        }
    }
}

impl std::fmt::Display for Direction4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction4::East => write!(f, "{}", EAST_NAME),
            Direction4::North => write!(f, "{}", NORTH_NAME),
            Direction4::West => write!(f, "{}", WEST_NAME),
            Direction4::South => write!(f, "{}", SOUTH_NAME),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("the direction name `{}` was not recogonized", .0)]
pub struct NoSuchDirectionNameError(String);

impl FromStr for Direction4 {
    type Err = NoSuchDirectionNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case(EAST_NAME) {
            Ok(Direction4::East)
        } else if s.eq_ignore_ascii_case(NORTH_NAME) {
            Ok(Direction4::North)
        } else if s.eq_ignore_ascii_case(WEST_NAME) {
            Ok(Direction4::West)
        } else if s.eq_ignore_ascii_case(SOUTH_NAME) {
            Ok(Direction4::South)
        } else {
            Err(NoSuchDirectionNameError(s.to_string()))
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

/// Represents east, north, west, south directions and the four diagonals
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
    /// Return an iterator over directions clockwise starting from East.
    ///
    /// ```
    /// use workshop::spatial::Direction8;
    ///
    /// for dir in Direction8::all() {
    ///   println!("{dir}");
    /// }
    ///
    /// assert_eq!(
    ///   Direction8::all().collect::<Vec<Direction8>>(),
    ///   [
    ///     Direction8::East,
    ///     Direction8::Northeast,
    ///     Direction8::North,
    ///     Direction8::Northwest,
    ///     Direction8::West,
    ///     Direction8::Southwest,
    ///     Direction8::South,
    ///     Direction8::Southeast,
    ///   ]
    /// );
    /// ```
    pub fn all() -> Direction8Itr {
        Direction8Itr {
            next: Some(Direction8::East),
        }
    }
}

impl std::fmt::Display for Direction8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction8::East => write!(f, "{}", EAST_NAME),
            Direction8::Northeast => write!(f, "{}", NORTHEAST_NAME),
            Direction8::North => write!(f, "{}", NORTH_NAME),
            Direction8::Northwest => write!(f, "{}", NORTHWEST_NAME),
            Direction8::West => write!(f, "{}", WEST_NAME),
            Direction8::Southwest => write!(f, "{}", SOUTHWEST_NAME),
            Direction8::South => write!(f, "{}", SOUTH_NAME),
            Direction8::Southeast => write!(f, "{}", SOUTHEAST_NAME),
        }
    }
}

impl FromStr for Direction8 {
    type Err = NoSuchDirectionNameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case(EAST_NAME) {
            Ok(Direction8::East)
        } else if s.eq_ignore_ascii_case(NORTHEAST_NAME) {
            Ok(Direction8::Northeast)
        } else if s.eq_ignore_ascii_case(NORTH_NAME) {
            Ok(Direction8::North)
        } else if s.eq_ignore_ascii_case(NORTHWEST_NAME) {
            Ok(Direction8::Northwest)
        } else if s.eq_ignore_ascii_case(WEST_NAME) {
            Ok(Direction8::West)
        } else if s.eq_ignore_ascii_case(SOUTHWEST_NAME) {
            Ok(Direction8::Southwest)
        } else if s.eq_ignore_ascii_case(SOUTH_NAME) {
            Ok(Direction8::South)
        } else if s.eq_ignore_ascii_case(SOUTHEAST_NAME) {
            Ok(Direction8::Southeast)
        } else {
            Err(NoSuchDirectionNameError(s.to_string()))
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
            Direction4::all().collect::<Vec<Direction4>>(),
            [
                Direction4::East,
                Direction4::North,
                Direction4::West,
                Direction4::South
            ]
        );
    }

    #[test]
    fn format_direction4() {
        assert_eq!(&format!("{}", Direction4::East), "East");
        assert_eq!(&format!("{}", Direction4::North), "North");
        assert_eq!(&format!("{}", Direction4::West), "West");
        assert_eq!(&format!("{}", Direction4::South), "South");
    }

    #[test]
    fn parse_direction() {
        assert_eq!("East".parse(), Ok(Direction4::East));
        assert_eq!("North".parse(), Ok(Direction4::North));
        assert_eq!("West".parse(), Ok(Direction4::West));
        assert_eq!("South".parse(), Ok(Direction4::South));
    }

    #[test]
    fn parse_direction_insensitive() {
        assert_eq!("east".parse(), Ok(Direction4::East));
        assert_eq!("nOrTh".parse(), Ok(Direction4::North));
        assert_eq!("WEST".parse(), Ok(Direction4::West));
        assert_eq!("sOUTh".parse(), Ok(Direction4::South));
    }

    #[test]
    fn parse_direction_unrecogonized() {
        assert_eq!(
            "E".parse::<Direction4>(),
            Err(NoSuchDirectionNameError("E".to_string()))
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
            Direction8::all().collect::<Vec<Direction8>>(),
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

    #[test]
    fn format_direction8() {
        assert_eq!(&format!("{}", Direction8::East), "East");
        assert_eq!(&format!("{}", Direction8::Northeast), "Northeast");
        assert_eq!(&format!("{}", Direction8::North), "North");
        assert_eq!(&format!("{}", Direction8::Northwest), "Northwest");
        assert_eq!(&format!("{}", Direction8::West), "West");
        assert_eq!(&format!("{}", Direction8::Southwest), "Southwest");
        assert_eq!(&format!("{}", Direction8::South), "South");
        assert_eq!(&format!("{}", Direction8::Southeast), "Southeast");
    }

    #[test]
    fn parse_direction8() {
        assert_eq!("East".parse(), Ok(Direction8::East));
        assert_eq!("Northeast".parse(), Ok(Direction8::Northeast));
        assert_eq!("North".parse(), Ok(Direction8::North));
        assert_eq!("Northwest".parse(), Ok(Direction8::Northwest));
        assert_eq!("West".parse(), Ok(Direction8::West));
        assert_eq!("Southwest".parse(), Ok(Direction8::Southwest));
        assert_eq!("South".parse(), Ok(Direction8::South));
        assert_eq!("Southeast".parse(), Ok(Direction8::Southeast));
    }

    #[test]
    fn parse_direction8_insensitive() {
        assert_eq!("east".parse(), Ok(Direction8::East));
        assert_eq!("northEAST".parse(), Ok(Direction8::Northeast));
        assert_eq!("nOrTh".parse(), Ok(Direction8::North));
        assert_eq!("NORTHwest".parse(), Ok(Direction8::Northwest));
        assert_eq!("WEST".parse(), Ok(Direction8::West));
        assert_eq!("sOuThWeSt".parse(), Ok(Direction8::Southwest));
        assert_eq!("sOUTh".parse(), Ok(Direction8::South));
        assert_eq!("sOUTheASt".parse(), Ok(Direction8::Southeast));
    }

    #[test]
    fn parse_direction8_unrecogonized() {
        assert_eq!(
            "E".parse::<Direction8>(),
            Err(NoSuchDirectionNameError("E".to_string()))
        );
    }
}
