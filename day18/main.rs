use std::collections::HashSet;

use aoc23::{Dimensions, Direction, Grid, Position};

fn main() {
    let edges = include_str!("test.txt")
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

    println!("Part 1: {}", dig(&edges, false));
    //println!("Part 2: {}", dig(&edges, true));
}

fn dig(edges: &[Edge], actual: bool) -> usize {
    // Calculate grid dimensions and start position for digging.
    let (mut r, mut c): (i128, i128) = (0, 0);
    let (mut max_r, mut max_c): (i128, i128) = (0, 0);
    let (mut min_r, mut min_c): (i128, i128) = (0, 0);
    for edge in edges.iter() {
        let length = if actual {
            edge.actual_length()
        } else {
            edge.length
        } as i128;
        match if actual {
            edge.actual_direction()
        } else {
            edge.direction
        } {
            Direction::N => {
                r -= length;
                min_r = min_r.min(r);
            }
            Direction::E => {
                c += length;
                max_c = max_c.max(c);
            }
            Direction::S => {
                r += length;
                max_r = max_r.max(r);
            }
            Direction::W => {
                c -= length;
                min_c = min_c.min(c);
            }
        }
    }
    let dimensions = Dimensions {
        r: (max_r - min_r) as usize + 1,
        c: (max_c - min_c) as usize + 1,
    };
    let mut start_position = Position {
        r: min_r.unsigned_abs() as usize,
        c: min_c.unsigned_abs() as usize,
    };
    dbg!(dimensions);
    dbg!(start_position);

    println!("0000000000000000");
    let mut grid: Grid<Ground> = Grid::from(dimensions);
    println!("1111111111111111");

    // Dig border trench.
    //*grid.cell_mut(start_position) = Ground::Trench { color: "#ffffff" };
    *grid.cell_mut(start_position) = Ground::Trench;
    for edge in edges.iter() {
        let mut end: Option<Position> = None;
        let (length, direction) = match actual {
            true => (edge.actual_length(), edge.actual_direction()),
            false => (edge.length, edge.direction),
        };
        for position in start_position.steps(length, direction, &grid) {
            //*grid.cell_mut(position) = Ground::Trench { color: edge.color };
            *grid.cell_mut(position) = Ground::Trench;
            end = Some(position);
        }
        start_position = end.unwrap();
    }
    println!("2222222222222222");

    //print_grid(&grid);

    // Find a cell inside the border.
    let mut inside: Option<Position> = None;
    for (row_i, row) in grid.inner.iter().enumerate() {
        let mut row = row.iter().enumerate();
        if row.any(|(_, x)| matches!(x, Ground::Trench)) {
            if let Some((column_i, Ground::Level)) = row.next() {
                inside = Some(Position::new(row_i, column_i));
                break;
            }
        }
    }
    let inside = inside.unwrap();
    dbg!(inside);

    // Fill the center inside the border trench.
    let mut to_dig = HashSet::from([inside]);
    while !to_dig.is_empty() {
        let mut next_to_dig = HashSet::new();
        for trench in to_dig.iter() {
            //*grid.cell_mut(*trench) = Ground::Trench { color: "#ffffff" };
            *grid.cell_mut(*trench) = Ground::Trench;
            for adjacent in trench.adjacent(&grid) {
                if *grid.cell(adjacent) == Ground::Level {
                    next_to_dig.insert(adjacent);
                }
            }
        }
        to_dig = next_to_dig;

        /*
        println!("{}", to_dig.len());
        println!("{to_dig:?}");
        grid.print(|p, v| {
            if to_dig.contains(&p) {
                "X".to_string()
            } else {
                v.to_string()
            }
        });
        */
    }

    //print_grid(&grid);

    let part1 = grid.cells().filter(|x| matches!(x, Ground::Trench)).count();

    let level = grid.cells().filter(|x| matches!(x, Ground::Level)).count();
    let area = dimensions.r * dimensions.c;
    println!("trench:{part1} level:{level} area:{area}");

    //println!("Part 1: {part1}");

    part1
}

/*
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
*/

#[derive(Debug)]
struct Edge {
    direction: Direction,
    length: usize,
    color: &'static str,
}

impl Edge {
    fn actual_length(&self) -> usize {
        usize::from_str_radix(&self.color[1..6], 16).unwrap()
    }

    fn actual_direction(&self) -> Direction {
        match self.color[6..7].parse::<u8>().unwrap() {
            0 => Direction::E,
            1 => Direction::S,
            2 => Direction::W,
            3 => Direction::N,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
enum Ground {
    /*
    Trench {
        color: &'static str,
    },
    */
    Trench,
    #[default]
    Level,
}

impl From<&Ground> for char {
    fn from(value: &Ground) -> Self {
        match value {
            Ground::Trench => '#',
            Ground::Level => '.',
        }
    }
}
