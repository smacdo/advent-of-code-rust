use std::{
    borrow::{Borrow, BorrowMut},
    str::FromStr,
};

use thiserror::Error;

use crate::spatial::Point2;

/// A two dimensional grid with each cell storing a single value.
///
/// The origin (0, 0) of the grid is at the top left with the positive X axis
/// extending right and the positive Y axis extending down. Each cell is
/// addressable with an integer (X, Y) and has a width and height of 1. This
/// means that a grid where `x_count = 3` an `y_count = 2` would look like:
///
/// +--------+--------+--------+
/// | (0, 0) | (1, 0) | (2, 0) |
/// +--------+--------+--------+
/// | (0, 1) | (1, 1) | (2, 1) |
/// +--------+--------+--------+
///
/// Cells are stored in row major order meaning each column in the first row
/// is stored before the second row, etc. The previously shown example would
/// be stored in the following fashion:
///
/// +---+---+---+
/// | 0 | 1 | 2 |
/// +---+---+---+  --> [ 0, 1, 2, 3, 4, 5 ]
/// | 3 | 4 | 5 |
/// +---+---+---+
#[derive(Clone, Debug, PartialEq)]
pub struct Grid<T: std::fmt::Display> {
    /// Array of values stored in the grid.
    cells: Vec<T>,
    /// The number of cells in the horizontal (column direction).
    x_count: usize,
    /// The number of cells in the vertical (row direction).
    y_count: usize,
    /// The number of cells to the right that the origin (0, 0) has been shifted
    /// from the top left.
    x_origin_offset: isize,
    /// The number of cells down that the origin (0, 0) has been shifted from
    /// the top left.
    y_origin_offset: isize,
}

/// Converts a 2d (x, y) index into a 1d array offset with the assumption that
/// the underlying array is row-major.
#[inline(always)]
pub fn array_offset(
    x: isize,
    y: isize,
    x_origin_offset: isize,
    y_origin_offset: isize,
    x_count: usize,
) -> usize {
    let ax = x + x_origin_offset;
    let ay = y + y_origin_offset;

    assert!(ax >= 0);
    assert!(ay >= 0);

    (ay as usize) * x_count + (ax as usize)
}

impl<T: Clone + Sized + std::fmt::Display> Grid<T> {
    /// Return a new grid with `x_count` cols and `y_count` rows and with each
    /// cell initialized to `default`.
    pub fn new(x_count: usize, y_count: usize, default: T) -> Self {
        assert!(x_count <= isize::MAX as usize);
        assert!(y_count <= isize::MAX as usize);

        Grid {
            cells: vec![default; x_count * y_count],
            x_count,
            y_count,
            x_origin_offset: 0,
            y_origin_offset: 0,
        }
    }
}

impl<T: std::fmt::Display> Grid<T> {
    /// Return a new grid of `y_count` rows and `x_count` cols where each cell
    /// value is taken from the iterator `vals` in row major order.
    pub fn with_values<I>(
        x_count: usize,
        y_count: usize,
        vals: I,
    ) -> Result<Self, IteratorItemCountError>
    where
        I: Iterator<Item = T>,
    {
        assert!(x_count <= isize::MAX as usize);
        assert!(y_count <= isize::MAX as usize);

        let cells: Vec<T> = vals.collect();

        if x_count * y_count == cells.len() {
            Ok(Grid {
                cells,
                y_count,
                x_count,
                x_origin_offset: 0,
                y_origin_offset: 0,
            })
        } else {
            Err(IteratorItemCountError {
                x_count,
                y_count,
                actual_len: cells.len(),
            })
        }
    }

    /// Return the number of cells in the horizontal direction (columns)
    /// present in the grid.
    pub fn x_count(&self) -> usize {
        self.x_count
    }

    /// Return the number of cells in the vertical direction (rows) present in
    /// the grid.
    pub fn y_count(&self) -> usize {
        self.y_count
    }

    // TODO:
    pub fn top_left_pos(&self) -> Point2 {
        Point2::new(-self.x_origin_offset, -self.y_origin_offset)
    }

    // TODO:
    pub fn bottom_right_pos(&self) -> Point2 {
        Point2::new(
            self.x_count as isize - self.x_origin_offset - 1,
            self.y_count as isize - self.y_origin_offset - 1,
        )
    }

    // TODO: Comment and unit test.
    pub fn is_pos_in_bounds(&self, p: Point2) -> bool {
        p.x >= self.top_left_pos().x
            && p.x <= self.bottom_right_pos().x
            && p.y >= self.top_left_pos().y
            && p.y <= self.bottom_right_pos().y
    }

    /// Get a reference to the value stored at the given `x` column and `y` row.
    pub fn get(&'_ self, x: isize, y: isize) -> &'_ T {
        debug_assert!(x < self.x_count as isize - self.x_origin_offset);
        debug_assert!(y < self.y_count as isize - self.y_origin_offset);

        &self.cells[array_offset(
            x,
            y,
            self.x_origin_offset,
            self.y_origin_offset,
            self.x_count,
        )]
    }

    /// Get a mutable reference to the value stored at the given `x` column
    /// and `y` row.
    pub fn get_mut(&'_ mut self, x: isize, y: isize) -> &'_ mut T {
        debug_assert!(x < self.x_count as isize - self.x_origin_offset);
        debug_assert!(y < self.y_count as isize - self.y_origin_offset);

        &mut self.cells[array_offset(
            x,
            y,
            self.x_origin_offset,
            self.y_origin_offset,
            self.x_count,
        )]
    }

    /// Set the value stored at the given `x` column and `y` row.
    pub fn set(&mut self, x: isize, y: isize, value: T) {
        debug_assert!(x < self.x_count as isize - self.x_origin_offset);
        debug_assert!(y < self.y_count as isize - self.y_origin_offset);

        self.cells[array_offset(
            x,
            y,
            self.x_origin_offset,
            self.y_origin_offset,
            self.x_count,
        )] = value
    }

    /// Get an iterator to the values stored in the grid.
    ///
    /// This iterator will iterate row starting from at the first (top most)
    /// row, and iterate through every column left to right before proceeding to
    /// the next row.
    pub fn iter(&self) -> RegionItr<T> {
        let topleft_x = -self.x_origin_offset;
        let topleft_y = -self.y_origin_offset;

        RegionItr {
            grid: self,
            next_x: topleft_x,
            next_y: topleft_y,
            start_x: topleft_x,
            end_x: topleft_x + (self.x_count as isize),
            end_y: topleft_y + (self.y_count as isize),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..(self.y_count as isize) {
            for x in 0..(self.x_count as isize) {
                write!(f, "{}", self.get(x, y))?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T: std::fmt::Display> std::iter::IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

#[derive(Debug, Error)]
#[error("a grid of {} rows and {} cols requires {} values but the iterator produced {}", x_count, y_count, x_count * y_count, actual_len)]
pub struct IteratorItemCountError {
    x_count: usize,
    y_count: usize,
    actual_len: usize,
}

/// Converts a slice of strings into a 2d grid.
///
/// The length of each string in the slice is expected to be identical, otherwise
/// an error will be returned.
impl TryFrom<&[&str]> for Grid<char> {
    type Error = IteratorItemCountError;

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Ok(Grid::new(0, 0, Default::default()))
        } else {
            let itr = value.iter().flat_map(|s| s.chars());
            Self::with_values(value[0].len(), value.len(), itr)
        }
    }
}

/// Converts a multiline string into a 2d grid.
///
/// The length of each string in the slice is expected to be identical, otherwise
/// an error will be returned.
impl FromStr for Grid<char> {
    type Err = IteratorItemCountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Grid::new(0, 0, Default::default()))
        } else {
            let mut x_count: Option<usize> = None;
            let mut y_count = 0;

            for line in s.lines() {
                x_count = x_count.or(Some(line.len()));
                y_count += 1;
            }

            Self::with_values(
                x_count.unwrap_or(0),
                y_count,
                s.lines().flat_map(|line| line.chars()),
            )
        }
    }
}

/// A reference to a specific cell contained in a grid along with the (x, y)
/// position of the cell.
#[derive(Debug, PartialEq)]
pub struct CellRef<'a, T> {
    /// The position of the cell in the grid.
    pub index: Point2,
    /// A reference to the value stored in this grid cell.
    pub value: &'a T,
}

/// An iterator capable of iterating a bounded subregion of a larger grid. This
/// iterator is "heavier" than the typical iterator as it has to hold additional
/// information about starting and ending positions.
///
/// Currently this iterator is also used for the typical "iterate all cells in
/// the grid" functionality but if profiling shows this is a problem a new iter
/// can be created that only needs the current and row size.
pub struct RegionItr<'a, T: std::fmt::Display> {
    grid: &'a Grid<T>,
    next_x: isize,
    next_y: isize,
    start_x: isize,
    end_x: isize,
    end_y: isize,
}

impl<'a, T: Clone + std::fmt::Display> Iterator for RegionItr<'a, T> {
    type Item = CellRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_y >= self.end_y {
            None
        } else {
            let x = self.next_x;
            let y = self.next_y;

            self.next_x += 1;

            if self.next_x >= self.end_x {
                self.next_x = self.start_x;
                self.next_y += 1;
            }

            Some(CellRef {
                index: Point2::new(x, y),
                value: self.grid.get(x, y),
            })
        }
    }
}

impl<T: Clone + std::fmt::Display> std::ops::Index<Point2> for Grid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, p: Point2) -> &Self::Output {
        debug_assert!(p.x < self.x_count as isize - self.x_origin_offset);
        debug_assert!(p.y < self.y_count as isize - self.y_origin_offset);

        self.cells[array_offset(
            p.x,
            p.y,
            self.x_origin_offset,
            self.y_origin_offset,
            self.x_count,
        )]
        .borrow()
    }
}

impl<T: Clone + std::fmt::Display> std::ops::IndexMut<Point2> for Grid<T> {
    #[inline(always)]
    fn index_mut(&mut self, p: Point2) -> &mut Self::Output {
        debug_assert!(p.x < self.x_count as isize - self.x_origin_offset);
        debug_assert!(p.y < self.y_count as isize - self.y_origin_offset);

        self.cells[array_offset(
            p.x,
            p.y,
            self.x_origin_offset,
            self.y_origin_offset,
            self.x_count,
        )]
        .borrow_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_value_constructor() {
        let g: Grid<i32> = Grid::new(3, 2, 17);
        assert_eq!(2, g.y_count());
        assert_eq!(3, g.x_count());

        assert_eq!(17, g[Point2::new(0, 0)]);
        assert_eq!(17, g[Point2::new(1, 0)]);
        assert_eq!(17, g[Point2::new(2, 0)]);

        assert_eq!(17, g[Point2::new(0, 1)]);
        assert_eq!(17, g[Point2::new(1, 1)]);
        assert_eq!(17, g[Point2::new(2, 1)]);
    }

    #[test]
    fn zero_size_grid() {
        let g: Grid<i32> = Grid::new(0, 0, 13);
        assert_eq!(0, g.y_count());
        assert_eq!(0, g.x_count());
    }

    #[test]
    fn array_constructor() {
        let g: Grid<i32> = Grid::with_values(3, 2, [10, 20, 30, 40, 50, 60].into_iter()).unwrap();

        assert_eq!(10, g[Point2::new(0, 0)]);
        assert_eq!(20, g[Point2::new(1, 0)]);
        assert_eq!(30, g[Point2::new(2, 0)]);

        assert_eq!(40, g[Point2::new(0, 1)]);
        assert_eq!(50, g[Point2::new(1, 1)]);
        assert_eq!(60, g[Point2::new(2, 1)]);
    }

    #[test]
    fn array_constructor_with_zero_size() {
        let g = Grid::<i32>::with_values(0, 0, [].into_iter());
        assert!(g.is_ok());
    }

    #[test]
    fn set_values() {
        let mut g: Grid<i32> = Grid::new(3, 2, 0);

        g.set(2, 0, 42);
        assert_eq!(&42, g.get(2, 0));

        assert_eq!(&0, g.get(0, 1));
        g.set(0, 1, 22);

        assert_eq!(&22, g.get(0, 1));
    }

    #[test]
    fn get_mut_values() {
        let mut g: Grid<i32> = Grid::new(3, 2, 0);

        *g.get_mut(2, 0) = 42;
        assert_eq!(&42, g.get(2, 0));

        *g.get_mut(0, 1) = 2;
        assert_eq!(&2, g.get(0, 1));
    }

    #[test]
    fn index() {
        let mut g: Grid<i32> = Grid::new(3, 2, 0);

        g.set(2, 0, 42);
        assert_eq!(42, g[Point2::new(2, 0)]);

        assert_eq!(0, g[Point2::new(0, 1)]);

        g.set(0, 1, 22);
        assert_eq!(22, g[Point2::new(0, 1)]);
    }

    #[test]
    fn index_mut() {
        let mut g: Grid<i32> = Grid::new(3, 2, 0);

        g[Point2::new(2, 0)] = 42;
        assert_eq!(42, g[Point2::new(2, 0)]);

        assert_eq!(0, g[Point2::new(0, 1)]);
        g[Point2::new(0, 1)] = 22;

        assert_eq!(22, g[Point2::new(0, 1)]);
    }

    #[test]
    #[should_panic]
    fn panic_if_grid_out_of_bounds() {
        let g: Grid<i32> = Grid::new(4, 6, 0);
        assert_eq!(0, g[Point2::new(5, 5)]);
    }

    #[test]
    fn into_iter() {
        let g: Grid<i32> = Grid::with_values(3, 2, [10, 20, 30, 40, 50, 60].into_iter()).unwrap();
        let mut iter = g.into_iter();

        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(30));
        assert_eq!(iter.next(), Some(40));
        assert_eq!(iter.next(), Some(50));
        assert_eq!(iter.next(), Some(60));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn from_string_array() {
        let s = ["ABC", "123"];
        let g = Grid::try_from(s.as_slice()).unwrap();

        assert_eq!('A', g[Point2::new(0, 0)]);
        assert_eq!('B', g[Point2::new(1, 0)]);
        assert_eq!('C', g[Point2::new(2, 0)]);

        assert_eq!('1', g[Point2::new(0, 1)]);
        assert_eq!('2', g[Point2::new(1, 1)]);
        assert_eq!('3', g[Point2::new(2, 1)]);
    }

    #[test]
    fn from_string_array_empty() {
        let s = [];
        let g = Grid::try_from(s.as_slice()).unwrap();

        assert_eq!(g.y_count(), 0);
        assert_eq!(g.x_count(), 0);
    }

    #[test]
    fn from_string_array_too_small() {
        let s = ["ABC", "12"];
        assert!(matches!(
            Grid::try_from(s.as_slice()),
            Err(IteratorItemCountError {
                x_count: 3,
                y_count: 2,
                actual_len: 5
            })
        ));
    }

    #[test]
    fn from_string_array_too_big() {
        let s = ["ABC1", "125", "125"];
        assert!(matches!(
            Grid::try_from(s.as_slice()),
            Err(IteratorItemCountError {
                x_count: 4,
                y_count: 3,
                actual_len: 10
            })
        ));
    }

    #[test]
    fn from_string() {
        let s = "xyz\nijk";
        let g = Grid::<char>::from_str(s).unwrap();

        assert_eq!(g.x_count(), 3);
        assert_eq!(g.y_count(), 2);

        assert_eq!('x', g[Point2::new(0, 0)]);
        assert_eq!('y', g[Point2::new(1, 0)]);
        assert_eq!('z', g[Point2::new(2, 0)]);

        assert_eq!('i', g[Point2::new(0, 1)]);
        assert_eq!('j', g[Point2::new(1, 1)]);
        assert_eq!('k', g[Point2::new(2, 1)]);
    }

    #[test]
    fn from_empty_string() {
        let s = "";
        let g = Grid::<char>::from_str(s).unwrap();

        assert_eq!(g.y_count(), 0);
        assert_eq!(g.x_count(), 0);
    }

    #[test]
    fn from_string_too_small() {
        let s = "ABC\n12";
        assert!(matches!(
            Grid::<char>::from_str(s),
            Err(IteratorItemCountError {
                x_count: 3,
                y_count: 2,
                actual_len: 5
            })
        ));
    }

    #[test]
    fn from_string_too_big() {
        let s = "ABC1\n125\n125";
        assert!(matches!(
            Grid::<char>::from_str(s),
            Err(IteratorItemCountError {
                x_count: 4,
                y_count: 3,
                actual_len: 10
            })
        ));
    }

    #[test]
    fn print_grid() {
        let g = Grid::with_values(4, 3, "ABCD1234abcd".chars()).unwrap();
        assert_eq!(format!("{}", g), "ABCD\n1234\nabcd\n");
    }
}
