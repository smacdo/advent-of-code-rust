use std::{ops, str::FromStr};

use thiserror::Error;

use super::{Direction4, Direction8};

/// Represents an integer (x,y) cartesian point in two dimensions.
///
/// Some useful features in this type include the ability to convert two and
/// from `(isize, isize)` tuples.
#[derive(Clone, Copy, Debug, Default, Hash, Eq, PartialEq)]
pub struct Point2 {
    /// The x coordinate of this point.
    pub x: isize,
    /// The y coordinate of this point.
    pub y: isize,
}

impl Point2 {
    /// Initialize a new point with the given `x` and `y` values.
    #[inline]
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Return a point that has both `x` and `y` components set to zero.
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    /// Return a point that has both `x` and `y` components set to one.
    pub fn one() -> Self {
        Self { x: 1, y: 1 }
    }

    /// Return a point that has `x` set to one, and `y` set to zero.
    pub fn unit_x() -> Self {
        Self { x: 1, y: 0 }
    }

    /// Return a point that has `x` set to zero, and `y` set to one.
    pub fn unit_y() -> Self {
        Self { x: 0, y: 1 }
    }

    /// Return a vector that contains the largest value from each matching pair
    /// of components.
    pub fn max(a: Self, b: Self) -> Self {
        Self {
            x: isize::max(a.x, b.x),
            y: isize::max(a.y, b.y),
        }
    }

    /// Return a vector that contains the smallest value from each matching pair
    /// of components.
    pub fn min(a: Self, b: Self) -> Self {
        Self {
            x: isize::min(a.x, b.x),
            y: isize::min(a.y, b.y),
        }
    }

    /// Return a vector with the absolute `x` and `y` component values of this
    /// vector.
    pub fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

// Convert `(isize, isize)` to `Point2`.
impl From<(isize, isize)> for Point2 {
    fn from(value: (isize, isize)) -> Self {
        Point2 {
            x: value.0,
            y: value.1,
        }
    }
}

// `[usize]` operator.
impl ops::Index<usize> for Point2 {
    type Output = isize;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("invalid component index"),
        }
    }
}

// `[usize]` mut operator.
impl ops::IndexMut<usize> for Point2 {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("invalid component index"),
        }
    }
}

// Addition operator.
impl ops::Add for Point2 {
    type Output = Self;

    #[inline(always)]
    fn add(self, r: Self) -> Self {
        Self {
            x: self.x + r.x,
            y: self.y + r.y,
        }
    }
}

impl ops::Add<&Point2> for Point2 {
    type Output = Self;

    #[inline(always)]
    fn add(self, r: &Self) -> Self {
        Self {
            x: self.x + r.x,
            y: self.y + r.y,
        }
    }
}

// TODO: Not sure if adding a direction is best, or this could should be removed
//       in favor of direction * Point2::one.
impl ops::Add<Direction4> for Point2 {
    type Output = Self;

    #[inline(always)]
    fn add(self, d: Direction4) -> Self {
        let r: Point2 = d.into();
        self + r
    }
}

impl ops::Add<Direction8> for Point2 {
    type Output = Self;

    #[inline(always)]
    fn add(self, d: Direction8) -> Self {
        let r: Point2 = d.into();
        self + r
    }
}

// Self addition operator.
impl ops::AddAssign for Point2 {
    #[inline(always)]
    fn add_assign(&mut self, r: Self) {
        self.x += r.x;
        self.y += r.y;
    }
}

impl ops::AddAssign<&Point2> for Point2 {
    #[inline(always)]
    fn add_assign(&mut self, r: &Self) {
        self.x += r.x;
        self.y += r.y;
    }
}

// Subtaction operator.
impl ops::Sub<Point2> for Point2 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, r: Self) -> Self {
        Self {
            x: self.x - r.x,
            y: self.y - r.y,
        }
    }
}

impl ops::Sub<&Point2> for Point2 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, r: &Self) -> Self {
        Self {
            x: self.x - r.x,
            y: self.y - r.y,
        }
    }
}

// Self substraction operator.
impl ops::SubAssign for Point2 {
    #[inline(always)]
    fn sub_assign(&mut self, r: Self) {
        self.x -= r.x;
        self.y -= r.y;
    }
}

impl ops::SubAssign<&Point2> for Point2 {
    #[inline(always)]
    fn sub_assign(&mut self, r: &Self) {
        self.x -= r.x;
        self.y -= r.y;
    }
}

// Multiply by scalar operator.
impl ops::Mul<isize> for Point2 {
    type Output = Point2;

    #[inline(always)]
    fn mul(self, r: isize) -> Self {
        Self {
            x: self.x * r,
            y: self.y * r,
        }
    }
}

impl ops::Mul<Point2> for isize {
    type Output = Point2;

    #[inline(always)]
    fn mul(self, r: Point2) -> Point2 {
        Point2 {
            x: self * r.x,
            y: self * r.y,
        }
    }
}

impl ops::Mul<&Point2> for isize {
    type Output = Point2;

    #[inline(always)]
    fn mul(self, r: &Point2) -> Point2 {
        Point2 {
            x: self * r.x,
            y: self * r.y,
        }
    }
}

// Self multiply by scalar operator.
impl ops::MulAssign<isize> for Point2 {
    #[inline(always)]
    fn mul_assign(&mut self, r: isize) {
        self.x *= r;
        self.y *= r;
    }
}

// Divide by scalar operator.
impl ops::Div<isize> for Point2 {
    type Output = Self;

    #[inline(always)]
    fn div(self, r: isize) -> Self {
        Self {
            x: self.x / r,
            y: self.y / r,
        }
    }
}

// Self divide by scalar operator.
impl ops::DivAssign<isize> for Point2 {
    #[inline(always)]
    fn div_assign(&mut self, r: isize) {
        self.x /= r;
        self.y /= r;
    }
}

// Negation operator.
impl ops::Neg for Point2 {
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

// Custom comparison ordering that clusters points by their y component rather
// the x component.
impl Ord for Point2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // This is a custom cmp implementation so points can be clustered by
        // y value (not x) and still retain the struct {x, y} layout.
        match self.y.cmp(&other.y) {
            core::cmp::Ordering::Equal => self.x.cmp(&other.x),
            ord => ord,
        }
    }
}

impl PartialOrd for Point2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Display implementation that formats points as `(x, y)`.
impl std::fmt::Display for Point2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("the value `{}` is not a valid Point2", .0)]
pub struct ParsePointError(String);

impl FromStr for Point2 {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParsePointError(s.to_string()))?;

        let x = x
            .trim()
            .parse::<isize>()
            .map_err(|_| ParsePointError(s.to_string()))?;
        let y = y
            .trim()
            .parse::<isize>()
            .map_err(|_| ParsePointError(s.to_string()))?;

        Ok(Point2 { x, y })
    }
}

/// Iterates all points in the region formed by `a` and `b` corners. Typically
/// `a` would be the upper left corner, and `b` would be the bottom right corner.
#[allow(dead_code)]
pub fn iter_rows_inclusive(a: Point2, b: Point2) -> impl Iterator<Item = Point2> {
    let start_x = a.x.min(b.x);
    let start_y = a.y.min(b.y);
    let end_x = a.x.max(b.x) + 1;
    let end_y = a.y.max(b.y) + 1;

    (start_y..end_y).flat_map(move |y| (start_x..end_x).map(move |x| Point2::new(x, y)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_constructor() {
        let v = Point2::new(22, 15);
        assert_eq!(22, v.x);
        assert_eq!(15, v.y);
    }

    #[test]
    fn component_index() {
        let t = Point2::new(10, 20);
        assert_eq!(10, t[0]);
        assert_eq!(20, t[1]);
    }

    #[test]
    fn set_components_via_index() {
        let mut t = Point2::new(10, 20);
        t[0] = 15;
        t[1] = 25;

        assert_eq!(15, t[0]);
        assert_eq!(25, t[1]);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn add() {
        let a = Point2::new(1, -5);
        let b = Point2::new(-2, -3);
        let e = Point2::new(-1, -8);

        assert_eq!(e, a + b);
        assert_eq!(e, a + &b);

        let mut a1 = a;
        a1 += b;

        assert_eq!(e, a1);

        let mut a2 = a;
        a2 += &b;

        assert_eq!(e, a2);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn sub() {
        let a = Point2::new(1, -5);
        let b = Point2::new(-2, -3);
        let e = Point2::new(3, -2);

        assert_eq!(e, a - b);
        assert_eq!(e, a - &b);

        let mut a1 = a;
        a1 -= b;

        assert_eq!(e, a1);

        let mut a2 = a;
        a2 -= &b;

        assert_eq!(e, a2);
    }

    #[test]
    #[allow(clippy::op_ref)]
    fn mul() {
        let a = Point2::new(1, -5);
        let b = 3;
        let e = Point2::new(3, -15);

        assert_eq!(e, a * b);
        assert_eq!(e, b * a);
        assert_eq!(e, b * &a);

        let mut a1 = a;
        a1 *= b;

        assert_eq!(e, a1);
    }

    #[test]
    fn div() {
        let a = Point2::new(1, -5);
        let b = 2;
        let e = Point2::new(0, -2);

        assert_eq!(e, a / b);

        let mut a1 = a;
        a1 /= b;

        assert_eq!(e, a1);
    }

    #[test]
    fn negate() {
        let a = Point2::new(1, -5);
        let e = Point2::new(-1, 5);
        assert_eq!(e, -a);
    }

    #[test]
    fn get_xy() {
        let a = Point2::new(5, 7);
        assert_eq!(a.x, 5);
        assert_eq!(a.y, 7);
    }

    #[test]
    fn default_point_is_zeroed() {
        let a: Point2 = Default::default();
        assert_eq!(a.x, 0);
        assert_eq!(a.y, 0);
    }

    #[test]
    fn convert_from_2_tuple() {
        let p = Point2::from((72, 1351));
        assert_eq!(p.x, 72);
        assert_eq!(p.y, 1351);
    }

    #[test]
    fn convert_into_2_tuple() {
        let p: Point2 = (72, 1351).into();
        assert_eq!(p.x, 72);
        assert_eq!(p.y, 1351);
    }

    #[test]
    fn point_equality() {
        assert_eq!(Point2::new(5, 3), Point2::new(5, 3));
        assert_ne!(Point2::new(5, 3), Point2::new(-9, 2));
        assert_ne!(Point2::new(5, 3), Point2::new(8, 3));
        assert_ne!(Point2::new(5, 3), Point2::new(5, -1));
        assert_ne!(Point2::new(5, 3), Point2::new(3, 5));
    }

    #[test]
    fn compare_points() {
        assert_eq!(
            core::cmp::Ordering::Equal,
            Point2::new(4, 5).cmp(&Point2::new(4, 5))
        );
        assert_eq!(
            Some(core::cmp::Ordering::Equal),
            Point2::new(4, 5).partial_cmp(&Point2::new(4, 5))
        );

        assert_eq!(
            core::cmp::Ordering::Less,
            Point2::new(9, 5).cmp(&Point2::new(4, 6))
        );
        assert_eq!(
            Some(core::cmp::Ordering::Less),
            Point2::new(9, 5).partial_cmp(&Point2::new(4, 6))
        );

        assert_eq!(
            core::cmp::Ordering::Less,
            Point2::new(4, 5).cmp(&Point2::new(5, 5))
        );
        assert_eq!(
            Some(core::cmp::Ordering::Less),
            Point2::new(4, 5).partial_cmp(&Point2::new(5, 5))
        );

        assert_eq!(
            core::cmp::Ordering::Greater,
            Point2::new(6, 5).cmp(&Point2::new(5, 5))
        );
        assert_eq!(
            Some(core::cmp::Ordering::Greater),
            Point2::new(6, 5).partial_cmp(&Point2::new(5, 5))
        );

        assert_eq!(
            core::cmp::Ordering::Greater,
            Point2::new(0, 6).cmp(&Point2::new(4, 5))
        );
        assert_eq!(
            Some(core::cmp::Ordering::Greater),
            Point2::new(0, 6).partial_cmp(&Point2::new(4, 5))
        );
    }

    #[test]
    fn can_format_points() {
        assert_eq!("(2, -49)", format!("{}", Point2::new(2, -49)));
    }

    #[test]
    fn parse_points() {
        assert_eq!(Ok(Point2::new(2, -49)), "(2,-49)".parse());
        assert_eq!(Ok(Point2::new(2, -49)), "(2, -49)".parse());

        assert_eq!(
            Err(ParsePointError("2, -49".to_string())),
            "2, -49".parse::<Point2>()
        );
    }

    #[test]
    fn zero_point() {
        assert_eq!(Point2::new(0, 0), Point2::zero());
    }

    #[test]
    fn one_point() {
        assert_eq!(Point2::new(1, 1), Point2::one());
    }

    #[test]
    fn point_unit_x() {
        assert_eq!(Point2::new(1, 0), Point2::unit_x());
    }

    #[test]
    fn point_unit_y() {
        assert_eq!(Point2::new(0, 1), Point2::unit_y());
    }

    #[test]
    fn point2_supports_send_and_sync() {
        fn assert_send<T: Send>() {}
        assert_send::<Point2>();

        fn assert_sync<T: Sync>() {}
        assert_sync::<Point2>();
    }

    #[test]
    fn point2_abs() {
        assert_eq!(Point2::new(7, 13), Point2::new(7, 13).abs());
        assert_eq!(Point2::new(7, 13), Point2::new(-7, -13).abs());
        assert_eq!(Point2::new(7, 13), Point2::new(7, -13).abs());
        assert_eq!(Point2::new(7, 13), Point2::new(-7, 13).abs());
    }

    #[test]
    fn point2_max() {
        assert_eq!(
            Point2::new(-5, 7),
            Point2::max(Point2::new(-9, -2), Point2::new(-5, 7))
        );
        assert_eq!(
            Point2::new(2, 1),
            Point2::max(Point2::new(2, -2), Point2::new(-5, 1))
        );
        assert_eq!(
            Point2::new(1, -10),
            Point2::max(Point2::new(1, -10), Point2::new(0, -100))
        );
    }

    #[test]
    fn point2_min() {
        assert_eq!(
            Point2::new(-9, -2),
            Point2::min(Point2::new(-9, -2), Point2::new(-5, 7))
        );
        assert_eq!(
            Point2::new(-5, -2),
            Point2::min(Point2::new(2, -2), Point2::new(-5, 1))
        );
        assert_eq!(
            Point2::new(0, -100),
            Point2::min(Point2::new(1, -10), Point2::new(0, -100))
        );
    }

    #[test]
    fn iter_rows_inclusive_region() {
        assert_eq!(
            iter_rows_inclusive(Point2::new(3, 5), Point2::new(5, 6)).collect::<Vec<Point2>>(),
            vec![
                Point2::new(3, 5),
                Point2::new(4, 5),
                Point2::new(5, 5),
                Point2::new(3, 6),
                Point2::new(4, 6),
                Point2::new(5, 6)
            ]
        );
    }

    #[test]
    fn iter_rows_inclusive_param_order_doesnt_matter() {
        assert_eq!(
            iter_rows_inclusive(Point2::new(3, 5), Point2::new(5, 6)).collect::<Vec<Point2>>(),
            iter_rows_inclusive(Point2::new(5, 6), Point2::new(3, 5)).collect::<Vec<Point2>>(),
        );
        assert_eq!(
            iter_rows_inclusive(Point2::new(3, 5), Point2::new(5, 6)).collect::<Vec<Point2>>(),
            iter_rows_inclusive(Point2::new(5, 5), Point2::new(3, 6)).collect::<Vec<Point2>>(),
        );
        assert_eq!(
            iter_rows_inclusive(Point2::new(3, 5), Point2::new(5, 6)).collect::<Vec<Point2>>(),
            iter_rows_inclusive(Point2::new(5, 6), Point2::new(3, 5)).collect::<Vec<Point2>>(),
        );
    }
}
