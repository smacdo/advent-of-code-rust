use std::{
    borrow::{Borrow, BorrowMut},
    iter::FusedIterator,
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

    // Returns the position of the top leftmost cell in the grid.
    pub fn top_left(&self) -> Point2 {
        Point2::new(-self.x_origin_offset, -self.y_origin_offset)
    }

    // Returns the position of the bottom rightmost cell in the grid.
    pub fn bottom_right(&self) -> Point2 {
        Point2::new(
            self.x_count as isize - self.x_origin_offset - 1,
            self.y_count as isize - self.y_origin_offset - 1,
        )
    }

    // Checks if `p` is a point contained in this grid.
    pub fn is_pos_in_bounds(&self, p: Point2) -> bool {
        p.x >= self.top_left().x
            && p.x <= self.bottom_right().x
            && p.y >= self.top_left().y
            && p.y <= self.bottom_right().y
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

    /// Returns an iterator to the cells stored in the grid.
    ///
    /// This iterator will iterate row starting from at the first (top most)
    /// row, and iterate through every column left to right before proceeding to
    /// the next row.
    pub fn iter(&self) -> Cells<T> {
        Cells {
            points: self.points(),
            grid: self,
        }
    }

    // Returns an iterator that iterates all of the points in this grid going
    // one row at a time left to right, starting at the top left and ending at
    // the bottom right.
    pub fn points(&self) -> Points {
        let topleft_x = -self.x_origin_offset;
        let topleft_y = -self.y_origin_offset;

        Points {
            next_x: topleft_x,
            next_y: topleft_y,
            start_x: topleft_x,
            end_x: topleft_x + (self.x_count as isize),
            end_y: topleft_y + (self.y_count as isize),
        }
    }

    /// Returns an iterator over all the rows in the grid.
    pub fn rows(&self) -> Rows {
        let top_left = self.top_left();
        let bottom_right = self.bottom_right();
        let dims = bottom_right - top_left + Point2::one();

        Rows::new(top_left, dims.x, dims.y)
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

/// Represents scenarios where an iterator is used to construct a Grid and
/// does not match the expected length, width or height.
///
/// Potential scenarios that will cause an IteratorItemCountError:
///   - The column is inconsistent between rows
///   - There are less than `row_count * col_count` items from the iterator.
///   - There are more than `row_count * col_count` items from the iterator.
#[derive(Debug, Error)]
#[error("a grid of {} rows and {} cols requires {} values but the iterator produced {}", x_count, y_count, x_count * y_count, actual_len)]
pub struct IteratorItemCountError {
    pub x_count: usize,
    pub y_count: usize,
    pub actual_len: usize,
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

/// An iterator over the points of a rectangular region in a grid.
///
/// This iterator iterates in row major ordering, meaning the iterator will
/// produce all the points a grid row before moving to the next row. Each row
/// is iterated left to right, and the rows are iterated top to bottom.
pub struct Points {
    next_x: isize,
    next_y: isize,
    start_x: isize,
    end_x: isize,
    end_y: isize,
}

impl Iterator for Points {
    type Item = Point2;

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

            Some(Point2::new(x, y))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.next_y >= self.end_y {
            (0, Some(0))
        } else {
            let len: usize = ((self.end_x - self.next_x)
                + (self.end_x - self.start_x) * (self.end_y - self.next_y - 1))
                .try_into()
                .unwrap();
            (len, Some(len))
        }
    }
}

impl FusedIterator for Points {}

/// A reference to a specific cell contained in a grid along with the (x, y)
/// position of the cell.
#[derive(Debug, PartialEq)]
pub struct CellRef<'a, T> {
    /// The position of the cell in the grid.
    pub index: Point2,
    /// A reference to the value stored in this grid cell.
    pub value: &'a T,
}

/// An iterator over the cells of a rectangular region in a grid.
///
/// See `PointsItr` for details on iteration order.
pub struct Cells<'a, T: std::fmt::Display> {
    points: Points,
    grid: &'a Grid<T>,
}

impl<'a, T: Clone + std::fmt::Display> Iterator for Cells<'a, T> {
    type Item = CellRef<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.points.next().map(|p| CellRef {
            index: p,
            value: &self.grid[p],
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.points.size_hint()
    }
}

impl<'a, T: Clone + std::fmt::Display> FusedIterator for Cells<'a, T> {}

/// An iterator over the rows in a grid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rows {
    start_x: isize,
    end_x: isize,
    next_y: isize,
    end_y: isize,
}

impl Rows {
    pub fn new(start: Point2, x_count: isize, y_count: isize) -> Self {
        assert!(x_count >= 0);
        assert!(y_count >= 0);

        Self {
            start_x: start.x,
            end_x: start.x + x_count,
            next_y: start.y,
            end_y: start.y + y_count,
        }
    }
}

impl Iterator for Rows {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_y >= self.end_y {
            None
        } else {
            let y = self.next_y;
            self.next_y += 1;

            Some(Row {
                y,
                next_x: self.start_x,
                end_x: self.end_x,
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len: usize = (self.end_y - self.next_y).max(0).try_into().unwrap();
        (len, Some(len))
    }
}

impl FusedIterator for Rows {}

/// An iterator over the points in a grid row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    y: isize,
    next_x: isize,
    end_x: isize,
}

impl Row {
    pub fn new(y: isize, start_x: isize, end_x: isize) -> Self {
        assert!(start_x <= end_x);
        Self {
            y,
            next_x: start_x,
            end_x,
        }
    }
}

impl Iterator for Row {
    type Item = Point2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_x >= self.end_x {
            None
        } else {
            let x = self.next_x;
            self.next_x += 1;

            Some(Point2::new(x, self.y))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len: usize = (self.end_x - self.next_x).max(0).try_into().unwrap();
        (len, Some(len))
    }
}

impl FusedIterator for Row {}

/// An iterator over the cols in a grid.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cols {
    next_x: isize,
    end_x: isize,
    start_y: isize,
    end_y: isize,
}

impl Cols {
    pub fn new(start: Point2, x_count: isize, y_count: isize) -> Self {
        assert!(x_count >= 0);
        assert!(y_count >= 0);

        Self {
            next_x: start.x,
            end_x: start.x + x_count,
            start_y: start.y,
            end_y: start.y + y_count,
        }
    }
}

impl Iterator for Cols {
    type Item = Col;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_x >= self.end_x {
            None
        } else {
            let x = self.next_x;
            self.next_x += 1;

            Some(Col {
                x,
                next_y: self.start_y,
                end_y: self.end_y,
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len: usize = (self.end_x - self.next_x).max(0).try_into().unwrap();
        (len, Some(len))
    }
}

impl FusedIterator for Cols {}

/// An iterator over the points in a grid column.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Col {
    x: isize,
    next_y: isize,
    end_y: isize,
}

impl Col {
    pub fn new(x: isize, start_y: isize, end_y: isize) -> Self {
        assert!(start_y <= end_y);
        Self {
            x,
            next_y: start_y,
            end_y,
        }
    }
}

impl Iterator for Col {
    type Item = Point2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_y >= self.end_y {
            None
        } else {
            let y = self.next_y;
            self.next_y += 1;

            Some(Point2::new(self.x, y))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len: usize = (self.end_y - self.next_y).max(0).try_into().unwrap();
        (len, Some(len))
    }
}

impl FusedIterator for Col {}
