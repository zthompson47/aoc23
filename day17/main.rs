use aoc23::{Direction, Grid, Position};

fn main() {
    let mut grid = Grid::<Number>::from(include_str!("test.txt"));
    println!("{grid}");

    let start = Position::new(0, 0);
    let east = start
        .steps(3, Direction::E, &grid)
        .iter()
        .map(|position| {
            min_path(
                Phase {
                    position: *position,
                    direction: Direction::E,
                },
                &mut grid,
            )
        })
        .min()
        .unwrap();
    let south = start
        .steps(3, Direction::S, &grid)
        .iter()
        .map(|position| {
            min_path(
                Phase {
                    position: *position,
                    direction: Direction::S,
                },
                &mut grid,
            )
        })
        .min()
        .unwrap();
    let part1 = east.min(south);

    println!("Part 1: {part1}");
}

fn min_path(start: Phase, grid: &mut Grid<Number>) -> u32 {
    println!("{start:?}");
    for direction in Direction::all() {}
    0
}

#[derive(Debug)]
struct Phase {
    position: Position,
    direction: Direction,
    //steps_taken: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Number {
    inner: u32,
    visited_horizontal: bool,
    visited_vertical: bool,
}

impl From<char> for Number {
    fn from(value: char) -> Self {
        Number {
            inner: value.to_digit(10).unwrap(),
            visited_horizontal: false,
            visited_vertical: false,
        }
    }
}

impl From<&Number> for char {
    fn from(value: &Number) -> Self {
        value.inner.to_string().chars().collect::<Vec<_>>()[0]
    }
}
