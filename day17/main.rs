use std::{collections::HashMap, io::Write};

use log::debug;

use aoc23::{Alignment, Grid, Position};

fn main() {
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .target(env_logger::Target::Stdout)
        .init();
    let mut grid = Grid::<Number>::from(include_str!("input.txt"));
    println!("Part 1: {}", part1(&mut grid));
}

fn part1(grid: &mut Grid<Number>) -> u32 {
    let east_path = Path {
        last_corner: Corner {
            position: Position::new(0, 0),
            alignment: Alignment::Horizontal,
        },
        inner_heat: 0,
        step_heat: 0,
    };
    let south_path = Path {
        last_corner: Corner {
            position: Position::new(0, 0),
            alignment: Alignment::Vertical,
        },
        inner_heat: 0,
        step_heat: 0,
    };

    let mut cache = HashMap::new();
    find_min_path(east_path, grid, &mut cache);
    find_min_path(south_path, grid, &mut cache);

    let cache_vertical = cache.get(&Corner {
        position: grid.bottom_right(),
        alignment: Alignment::Vertical,
    });
    let cache_horizontal = cache.get(&Corner {
        position: grid.bottom_right(),
        alignment: Alignment::Horizontal,
    });

    *cache_vertical.min(cache_horizontal).unwrap()
}

fn min_solution_heat(grid: &Grid<Number>, cache: &HashMap<Corner, u32>) -> Option<u32> {
    let end_vertical = Corner {
        position: grid.bottom_right(),
        alignment: Alignment::Vertical,
    };
    let end_horizontal = Corner {
        position: grid.bottom_right(),
        alignment: Alignment::Horizontal,
    };
    cache
        .get(&end_vertical)
        .min(cache.get(&end_horizontal))
        .copied()
}

fn find_min_path(start: Path, grid: &Grid<Number>, cache: &mut HashMap<Corner, u32>) {
    // Stop paths with more heat than current solution.
    if let Some(min_heat) = min_solution_heat(grid, cache) {
        if start.inner_heat > min_heat {
            return;
        }
    }

    // Stop paths with more heat than cached heat on this corner.
    if let Some(cached_min) = cache.get(&start.last_corner) {
        if start.inner_heat >= *cached_min {
            return;
        }
    }
    cache.insert(start.last_corner, start.inner_heat);

    if start.last_corner.position == grid.bottom_right() {
        debug!("got solution heat: {}", start.inner_heat);
    }

    for path in start.step(3, grid) {
        find_min_path(path, grid, cache);
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Corner {
    position: Position,
    alignment: Alignment,
}

#[derive(Debug)]
struct Path {
    last_corner: Corner,
    step_heat: u32,
    inner_heat: u32,
}

impl Path {
    fn step(self, count: usize, grid: &Grid<Number>) -> impl Iterator<Item = Path> + use<'_> {
        let directions = self.last_corner.alignment.directions();
        let alignment = self.last_corner.alignment.orthogonal();

        let inner_heat = self.inner_heat;
        let mut step_heat = 0;
        let first = self
            .last_corner
            .position
            .steps(count, directions[0], grid)
            .into_iter()
            .map(move |position| {
                step_heat += grid.cell(position).inner;
                let step_corner = Corner {
                    position,
                    alignment,
                };
                Path {
                    inner_heat: inner_heat + step_heat,
                    last_corner: step_corner,
                    step_heat,
                }
            });

        let inner_heat = self.inner_heat;
        let mut step_heat = 0;
        let second = self
            .last_corner
            .position
            .steps(count, directions[1], grid)
            .into_iter()
            .map(move |position| {
                step_heat += grid.cell(position).inner;
                let step_corner = Corner {
                    position,
                    alignment,
                };
                Path {
                    inner_heat: inner_heat + step_heat,
                    last_corner: step_corner,
                    step_heat,
                }
            });

        first.chain(second)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Path:")?;
        writeln!(f, "  step_heat: {}", self.step_heat)?;
        writeln!(f, "  heat: {}", self.inner_heat)?;
        writeln!(f, "  last_corner: {:?}", self.last_corner)?;
        writeln!(f)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Number {
    inner: u32,
    min_vertical: Option<u32>,
    min_horizontal: Option<u32>,
}

impl From<char> for Number {
    fn from(value: char) -> Self {
        Number {
            inner: value.to_digit(10).unwrap(),
            ..Default::default()
        }
    }
}

impl From<&Number> for char {
    fn from(value: &Number) -> Self {
        value.inner.to_string().chars().collect::<Vec<_>>()[0]
    }
}
