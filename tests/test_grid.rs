use std::str::FromStr;

use advent_of_code_rust::spatial::{Col, Cols, Grid, IteratorItemCountError, Point2, Row, Rows};

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
fn iter() {
    let g: Grid<i32> = Grid::with_values(3, 2, [10, 20, 30, 40, 50, 60].into_iter()).unwrap();
    let cells: Vec<(Point2, i32)> = g.iter().map(|c| (c.index, *c.value)).collect();

    assert_eq!(
        cells,
        vec![
            (Point2::new(0, 0), 10),
            (Point2::new(1, 0), 20),
            (Point2::new(2, 0), 30),
            (Point2::new(0, 1), 40),
            (Point2::new(1, 1), 50),
            (Point2::new(2, 1), 60),
        ]
    );
}

#[test]
fn points_iter() {
    let g: Grid<i32> = Grid::new(3, 2, 0);
    let points: Vec<Point2> = g.points().collect();

    assert_eq!(
        points,
        vec![
            Point2::new(0, 0),
            Point2::new(1, 0),
            Point2::new(2, 0),
            Point2::new(0, 1),
            Point2::new(1, 1),
            Point2::new(2, 1)
        ]
    );
}

#[test]
fn grid_rows_iter() {
    let g: Grid<i32> = Grid::new(3, 2, 0);
    let points: Vec<Point2> = g.rows().flatten().collect();

    assert_eq!(
        points,
        vec![
            Point2::new(0, 0),
            Point2::new(1, 0),
            Point2::new(2, 0),
            Point2::new(0, 1),
            Point2::new(1, 1),
            Point2::new(2, 1)
        ]
    );
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
fn rows_iter() {
    assert_eq!(
        Rows::new(Point2::new(3, 5), 2, 0).collect::<Vec<Row>>(),
        vec![]
    );

    assert_eq!(
        Rows::new(Point2::new(3, 5), 2, 3).collect::<Vec<Row>>(),
        vec![Row::new(5, 3, 5), Row::new(6, 3, 5), Row::new(7, 3, 5)]
    );
}

#[test]
fn row_iter() {
    assert_eq!(Row::new(-2, 2, 2).collect::<Vec<Point2>>(), vec![]);
    assert_eq!(
        Row::new(5, 12, 16).collect::<Vec<Point2>>(),
        vec![
            Point2::new(12, 5),
            Point2::new(13, 5),
            Point2::new(14, 5),
            Point2::new(15, 5),
        ]
    );
    assert_eq!(
        Row::new(3, -8, -5).collect::<Vec<Point2>>(),
        vec![Point2::new(-8, 3), Point2::new(-7, 3), Point2::new(-6, 3),]
    );
}

#[test]
fn cols_iter() {
    assert_eq!(
        Cols::new(Point2::new(3, 5), 0, 2).collect::<Vec<Col>>(),
        vec![]
    );

    assert_eq!(
        Cols::new(Point2::new(3, 5), 2, 3).collect::<Vec<Col>>(),
        vec![Col::new(3, 5, 8), Col::new(4, 5, 8)]
    );
}

#[test]
fn col_iter() {
    assert_eq!(Col::new(7, -3, -3).collect::<Vec<Point2>>(), vec![]);
    assert_eq!(
        Col::new(5, 12, 16).collect::<Vec<Point2>>(),
        vec![
            Point2::new(5, 12),
            Point2::new(5, 13),
            Point2::new(5, 14),
            Point2::new(5, 15),
        ]
    );
    assert_eq!(
        Col::new(3, -8, -5).collect::<Vec<Point2>>(),
        vec![Point2::new(3, -8), Point2::new(3, -7), Point2::new(3, -6),]
    );
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
