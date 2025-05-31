use advent_of_code_data::{Answer, Day, Year};
use advent_of_code_rust::spatial::{Direction4, Grid};
use linkme::distributed_slice;
use yuletide::{Example, Result, Solver, SolverPart};

use crate::SOLVERS;

#[distributed_slice(SOLVERS)]
static SOLVER: Solver = Solver {
    day: Day(15),
    year: Year(2024),
    part_one: SolverPart {
        func: day_15_1,
        examples: &[
            Example {
                input: "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
                expected: Answer::Int(2028),
            },
            Example {
                input: "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
                expected: Answer::Int(10092),
            },
        ],
    },
    part_two: SolverPart {
        func: day_15_2,
        examples: &[
            //(Answer::Int(0), "Example input",)
        ],
    },
};

#[derive(Debug, PartialEq)]
enum Tile {
    Floor,
    Wall,
    Box,
    Robot,
}

fn parse_input(input: &str) -> (Grid<Tile>, Vec<Direction4>) {
    // Split input into the tilemap and move list section.
    // TODO: Make this a utility function `split_first_blank_line`.
    // TODO: Also `split_blank_lines`.
    let (tilemap_str, command_str) =
        input.split_at(input.find("\n\n").or(input.find("\r\n\r\n")).unwrap());

    let tilemap: Grid<Tile> = Grid::parse_str(tilemap_str, |c| match c {
        '.' => Tile::Floor,
        '#' => Tile::Wall,
        'O' => Tile::Box,
        '@' => Tile::Robot,
        _ => panic!("unknown char when parsing tilemap"),
    })
    .unwrap();

    let moves = command_str
        .chars()
        .map(|c| match c {
            '>' => Direction4::East,
            '^' => Direction4::North,
            '<' => Direction4::West,
            'v' => Direction4::South,
            _ => panic!("unknown char when parsing moves"),
        })
        .collect();

    (tilemap, moves)
}

fn simulate(tilemap: &mut Grid<Tile>, _movement: Direction4) {
    let _robot_pos = tilemap.find(&Tile::Robot).unwrap();

    // Is there space in the direction the robot is moving?
    // Look past (skip over) boxes because those will be moved too.
    //let mut has_movable_space =
}

pub fn day_15_1(input: &str) -> Result<Answer> {
    let (mut tilemap, moves) = parse_input(input);

    for m in moves {
        simulate(&mut tilemap, m);
    }

    todo!("implement me! -- day15.rs:105");
}

pub fn day_15_2(_input: &str) -> Result<Answer> {
    Err(yuletide::SolverError::NotFinished)
}
