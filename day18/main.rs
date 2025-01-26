use std::collections::{BTreeMap, HashSet};

use aoc23::{Dimensions, Direction, Grid, Position};

fn main() {
    let edges = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            Edge {
                direction: Direction::from(parts.next().unwrap()),
                length: parts.next().unwrap().parse::<usize>().unwrap(),
                color: parts.next().unwrap().trim_matches(['(', ')']),
            }
        })
        .collect::<Vec<_>>();

    println!("Part 1: {}", dig(&edges, Part::Part1));
    println!("Part 2: {}", dig(&edges, Part::Part2));
}

#[derive(Clone, Copy, PartialEq)]
enum Orientation {
    Up,
    Down,
    Sideways,
}

fn dig(edges: &[Edge], part: Part) -> usize {
    // Set up sparse structure to hold edge data.
    let (dimensions, start_position) = dimensions_and_start(edges, part);
    let mut grid: Vec<BTreeMap<usize, Orientation>> = vec![BTreeMap::new(); dimensions.r];
    //dbg!(dimensions);
    //dbg!(start_position);
    //dbg!(grid.len());

    // Draw edges with orientation.
    let mut position = start_position;
    let mut end_orientation: Option<Orientation> = None;
    for edge in edges.iter() {
        let orientation = edge.orientation(part);
        for _ in 0..edge.length(part) {
            if orientation == Orientation::Up || orientation == Orientation::Down {
                grid[position.r].insert(position.c, orientation);
            }
            position = position.step_fallible(edge.direction(part));
            grid[position.r].insert(position.c, orientation);
        }
        end_orientation = Some(orientation);
    }

    // Set start point orientation based on orientation of final edge.
    if let Some(orientation) = end_orientation {
        if orientation == Orientation::Up || orientation == Orientation::Down {
            grid[start_position.r].insert(start_position.c, orientation);
        }
    }

    // Figure out which side of the edge orientation is the interior.
    // Assume there is a section of vertical edge on the left, one unit thick, with interior
    // directly to the right.
    let mut to_inside: Option<Orientation> = None;
    for row in grid.iter() {
        if let Some((column, orientation)) = row.first_key_value() {
            if [Orientation::Up, Orientation::Down].contains(orientation) {
                if let Some(next_column) = row.keys().nth(1) {
                    if next_column - column > 1 {
                        to_inside = Some(*orientation);
                        break;
                    }
                }
            }
        }
    }
    let to_inside = to_inside.unwrap();
    let to_outside = match to_inside {
        Orientation::Up => Orientation::Down,
        Orientation::Down => Orientation::Up,
        Orientation::Sideways => unreachable!(),
    };

    // Calculate area of trench.
    let mut result = 0;
    for row in grid.iter() {
        let mut prior_column: Option<usize> = None;
        for (column, orientation) in row {
            result += 1;
            if *orientation == to_outside {
                if let Some(prior_column) = prior_column {
                    result += column - prior_column - 1;
                }
            }
            prior_column = Some(*column);
        }
    }

    result

    /*
    // Print data structure.
    for r in 0..dimensions.r {
        for c in 0..dimensions.c {
            if let Some(orientation) = grid[r].get(&c) {
                print!(
                    "{}",
                    match orientation {
                        Orientation::Up => "^",
                        Orientation::Down => "v",
                        Orientation::Sideways => "#",
                    }
                );
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
    */
}

#[derive(Clone, Copy)]
enum Part {
    Part1,
    Part2,
}

fn dimensions_and_start(edges: &[Edge], part: Part) -> (Dimensions, Position) {
    type Signed = i32;

    let (mut r, mut c): (Signed, Signed) = (0, 0);
    let (mut max_r, mut max_c): (Signed, Signed) = (0, 0);
    let (mut min_r, mut min_c): (Signed, Signed) = (0, 0);

    for edge in edges.iter() {
        match edge.direction(part) {
            Direction::N => {
                r -= edge.length(part) as Signed;
                min_r = min_r.min(r);
            }
            Direction::E => {
                c += edge.length(part) as Signed;
                max_c = max_c.max(c);
            }
            Direction::S => {
                r += edge.length(part) as Signed;
                max_r = max_r.max(r);
            }
            Direction::W => {
                c -= edge.length(part) as Signed;
                min_c = min_c.min(c);
            }
        }
    }

    let dimensions = Dimensions {
        r: (max_r - min_r) as usize + 1,
        c: (max_c - min_c) as usize + 1,
    };
    let start_position = Position {
        r: min_r.unsigned_abs() as usize,
        c: min_c.unsigned_abs() as usize,
    };

    (dimensions, start_position)
}

#[allow(unused)]
fn dig_part1(edges: &[Edge]) -> usize {
    let (dimensions, mut start_position) = dimensions_and_start(edges, Part::Part1);
    let mut grid: Grid<Ground> = Grid::from(dimensions);

    // Dig border trench.
    *grid.cell_mut(start_position) = Ground::Trench { color: "#ffffff" };
    for edge in edges.iter() {
        let mut end: Option<Position> = None;
        for position in
            start_position.steps(edge.length(Part::Part1), edge.direction(Part::Part1), &grid)
        {
            *grid.cell_mut(position) = Ground::Trench { color: edge.color };
            end = Some(position);
        }
        start_position = end.unwrap();
    }

    print_grid(&grid);

    // Find a cell inside the border.
    let mut inside: Option<Position> = None;
    for (row_i, row) in grid.inner.iter().enumerate() {
        let mut row = row.iter().enumerate();
        if row.any(|(_, x)| matches!(x, Ground::Trench { .. })) {
            if let Some((column_i, Ground::Level)) = row.next() {
                inside = Some(Position::new(row_i, column_i));
                break;
            }
        }
    }
    let inside = inside.unwrap();

    // Fill the center inside the border trench.
    let mut to_dig = HashSet::from([inside]);
    while !to_dig.is_empty() {
        let mut next_to_dig = HashSet::new();
        for trench in to_dig.iter() {
            *grid.cell_mut(*trench) = Ground::Trench { color: "#ffffff" };
            for adjacent in trench.adjacent(&grid) {
                if *grid.cell(adjacent) == Ground::Level {
                    next_to_dig.insert(adjacent);
                }
            }
        }
        to_dig = next_to_dig;
    }

    //print_grid(&grid);

    grid.cells()
        .filter(|x| matches!(x, Ground::Trench { .. }))
        .count()
}

#[allow(unused)]
fn print_grid(grid: &Grid<Ground>) {
    grid.print(|position, value| {
        use colored::{ColoredString, Colorize};
        if let Ground::Trench { color } = grid.cell(position) {
            let r = u8::from_str_radix(&color[1..3], 16).unwrap();
            let g = u8::from_str_radix(&color[3..5], 16).unwrap();
            let b = u8::from_str_radix(&color[5..7], 16).unwrap();
            value.to_string().as_str().truecolor(r, g, b)
        } else {
            ColoredString::from(value.to_string())
        }
    });
}

#[derive(Debug)]
struct Edge {
    direction: Direction,
    length: usize,
    color: &'static str,
}

impl Edge {
    fn length(&self, part: Part) -> usize {
        match part {
            Part::Part1 => self.length,
            Part::Part2 => usize::from_str_radix(&self.color[1..6], 16).unwrap(),
        }
    }

    fn direction(&self, part: Part) -> Direction {
        match part {
            Part::Part1 => self.direction,
            Part::Part2 => match self.color[6..7].parse::<u8>().unwrap() {
                0 => Direction::E,
                1 => Direction::S,
                2 => Direction::W,
                3 => Direction::N,
                _ => unreachable!(),
            },
        }
    }

    fn orientation(&self, part: Part) -> Orientation {
        match self.direction(part) {
            Direction::N => Orientation::Up,
            Direction::S => Orientation::Down,
            Direction::E | Direction::W => Orientation::Sideways,
        }
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
enum Ground {
    Trench {
        color: &'static str,
    },
    #[default]
    Level,
}

impl From<&Ground> for char {
    fn from(value: &Ground) -> Self {
        match value {
            Ground::Trench { .. } => '#',
            Ground::Level => '.',
        }
    }
}
