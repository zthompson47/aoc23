use std::collections::HashMap;

use aoc23::{Alignment, Direction, Grid, Position};

fn main() {
    let mut grid = Grid::<Number>::from(include_str!("test.txt"));
    let start = Position::new(0, 0);
    let east_phase = Phase {
        position: start,
        direction: Direction::E,
    };
    let south_phase = Phase {
        position: start,
        direction: Direction::S,
    };
    let east_start = Path {
        phases: vec![east_phase],
        positions: vec![],
        max_heat: None,
        added_heat: 0,
    };
    let south_start = Path {
        phases: vec![south_phase],
        positions: vec![],
        max_heat: None,
        added_heat: 0,
    };

    let mut cache = HashMap::new();
    let east = find_min_path(&east_start, &mut grid, &mut cache);
    let mut cache = HashMap::new();
    let south = find_min_path(&south_start, &mut grid, &mut cache);

    /*
    let a = Some(234);
    let b = None;
    let c = a.min(b);
    let d = b.min(a);
    println!("{c:?} {d:?}");
    */

    let part1 = east.min(south).unwrap();
    println!("Part 1: {part1}");
}

fn find_min_path(
    start: &Path,
    grid: &mut Grid<Number>,
    cache: &mut HashMap<Phase, Option<u32>>,
) -> Option<u32> {
    let phase = start.phases.last().unwrap();

    println!("find_min_path: {phase:?}");
    grid.print(|position: Position, c: char| {
        if grid.cell(position).horizontal_min.is_some()
            || grid.cell(position).vertical_min.is_some()
        {
            "X".into()
        } else if cache
            .get(&Phase {
                position,
                direction: Direction::E,
            })
            .is_some()
            || cache
                .get(&Phase {
                    position,
                    direction: Direction::N,
                })
                .is_some()
            || cache
                .get(&Phase {
                    position,
                    direction: Direction::S,
                })
                .is_some()
            || cache
                .get(&Phase {
                    position,
                    direction: Direction::W,
                })
                .is_some()
        {
            "X".into()
        } else if start.positions.iter().any(|p| *p == position) {
            ".".into()
        } else {
            c.to_string()
        }
    });
    println!();

    if let Some(min_path) = cache.get(phase) {
        //println!("-------->>>>GOT CACHE {phase:?} = {min_path:?}");
        return *min_path;
    }

    if start.heat(grid) > 2000 {
        println!("MAX");
        return None;
    }

    if start.phases[0..start.phases.len() - 1].contains(phase) {
        println!("LOOP: {:?}", phase.position);
        return None;
    }

    if phase.position == grid.bottom_right() {
        println!("SOLUTION: {}", start.heat(grid));
        return Some(0);
    }

    let mut min_path: Option<u32> = None;
    for next_path in start.step(grid) {
        let mut next_min_path = find_min_path(&next_path, grid, cache);
        if next_min_path.is_some() {
            println!(
                "FOUND path {} + {}, {:?}",
                next_min_path.unwrap(),
                next_path.added_heat,
                next_path.phases.last()
            );
            next_min_path = Some(next_min_path.unwrap() + next_path.added_heat);
        }
        min_path = match (min_path, next_min_path) {
            (Some(min_path), Some(next_min_path)) => Some(min_path.min(next_min_path)),
            (Some(x), None) | (None, Some(x)) => Some(x),
            _ => None,
        };
        if min_path.is_some() {
            println!("CACHE min_path:{min_path:?}");
            cache.insert(*phase, min_path);
        }
    }
    println!("Got min: {min_path:?}");

    min_path
}

#[derive(Clone, Debug, PartialEq)]
struct Path {
    phases: Vec<Phase>,
    positions: Vec<Position>,
    max_heat: Option<u32>,
    added_heat: u32,
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Path: max_heat: {:?}", self.max_heat)?;
        for phase in self.phases.iter() {
            writeln!(
                f,
                "  phase [{}, {}] {:?}",
                phase.position.r, phase.position.c, phase.direction
            )?;
        }
        for position in self.positions.iter() {
            writeln!(f, "  position [{}, {}]", position.r, position.c)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Phase {
    position: Position,
    direction: Direction,
}

#[derive(Clone, Debug, PartialEq)]
struct Number {
    inner: u32,
    horizontal_min: Option<u32>,
    vertical_min: Option<u32>,
}

impl Path {
    fn heat(&self, grid: &Grid<Number>) -> u32 {
        self.positions.iter().map(|x| grid.cell(*x).inner).sum()
    }

    fn step(&self, grid: &Grid<Number>) -> Vec<Path> {
        let mut new_paths = Vec::new();
        for direction in self.phases.last().unwrap().direction.orthogonal() {
            let mut new_positions = Vec::new();
            for position in self
                .phases
                .last()
                .unwrap()
                .position
                .steps(3, direction, grid)
            {
                new_positions.push(position);
                let new_phase = Phase {
                    position,
                    direction,
                };
                let mut old_positions = self.positions.clone();
                old_positions.extend(new_positions.iter());

                let mut old_phases = self.phases.clone();
                old_phases.push(new_phase);
                let new_path = Path {
                    phases: old_phases,
                    positions: old_positions,
                    max_heat: None,
                    added_heat: new_positions.iter().map(|x| grid.cell(*x).inner).sum(),
                };
                new_paths.push(new_path);
            }
        }
        new_paths
    }
}

impl From<char> for Number {
    fn from(value: char) -> Self {
        Number {
            inner: value.to_digit(10).unwrap(),
            horizontal_min: None,
            vertical_min: None,
        }
    }
}

impl From<&Number> for char {
    fn from(value: &Number) -> Self {
        value.inner.to_string().chars().collect::<Vec<_>>()[0]
    }
}
