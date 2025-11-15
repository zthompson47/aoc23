use std::collections::HashSet;

use aoc23::{Direction, Grid, Position};
use grid_derive::Grid;

fn main() {
    let input = include_str!("test.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = Grid::<Tile>::from(input);
    let start = Position::new(0, 1);
    let visited: HashSet<Position> = HashSet::new();
    let mut solutions = Vec::new();
    find_longest_path(&grid, start, visited, true, &mut solutions).unwrap()
    //let old = find_longest_path(&grid, start, visited, true, &mut solutions).unwrap();
    //*solutions.iter().max().unwrap()
}

fn part2(input: &str) -> usize {
    let grid = Grid::<Tile>::from(input);
    let start = Position::new(0, 1);
    let visited: HashSet<Position> = HashSet::new();
    let mut solutions = Vec::new();
    let old = find_longest_path(&grid, start, visited, false, &mut solutions).unwrap();
    *solutions.iter().max().unwrap()
}

fn find_longest_path(
    grid: &Grid<Tile>,
    mut current_tile: Position,
    mut visited: HashSet<Position>,
    slippery: bool,
    solutions: &mut Vec<usize>,
) -> Option<usize> {
    let mut steps = 0;
    loop {
        if !visited.insert(current_tile) {
            panic!()
        }
        match find_next_step(grid, current_tile, &visited, slippery) {
            NextStep::Tile(next_tile) => {
                current_tile = next_tile;
                steps += 1;
            }
            NextStep::Hub(positions) => {
                // Choose longest branch.
                steps += 1;
                let mut max_steps = 0;
                for position in positions {
                    if let Some(steps) =
                        find_longest_path(grid, position, visited.clone(), slippery, solutions)
                    {
                        //println!("==ps>> {position} {steps}");
                        max_steps = max_steps.max(steps);
                        //println!("==>> {position} {max_steps}\n");
                    }
                }
                //println!("==ms>> {steps} {max_steps}");
                steps += max_steps;
                break;
            }
            NextStep::Finish => {
                solutions.push(visited.len());
                steps += 1;
                /*grid.print(|p, c| {
                    if visited.contains(&p) && c == '.' {
                        'O'
                    } else {
                        c
                    }
                });
                println!("{}", visited.len());*/

                break;
            }
            NextStep::Error => {
                return None;
            }
        }
    }

    //println!("========{current_tile}=========================>>>> {steps}");
    Some(steps)
}

fn find_next_step(
    grid: &Grid<Tile>,
    tile: Position,
    visited: &HashSet<Position>,
    slippery: bool,
) -> NextStep {
    //println!("-->find_next_step {}", tile);
    let mut adjacent = tile.adjacent_if_direction(grid, |t, _| *t != Tile::Forest);

    //println!("VISITED: {visited:?}");
    //println!("ADJACENT before: {} {adjacent:?}", adjacent.len());
    adjacent.retain(|x| !visited.contains(&x.0));
    //println!("ADJACENT after: {} {adjacent:?}", adjacent.len());

    if adjacent.len() == 1 {
        if adjacent[0].0 == Position::new(grid.dim().r - 1, grid.dim().c - 2) {
            NextStep::Finish
        } else {
            NextStep::Tile(adjacent[0].0)
        }
    } else if adjacent.iter().all(|x| {
        [
            Tile::SlideSouth,
            Tile::SlideWest,
            Tile::SlideEast,
            Tile::SlideNorth,
        ]
        .contains(grid.cell(x.0))
    }) {
        let mut tiles = vec![];
        for a in adjacent {
            if !slippery
                || *grid.cell(a.0) == Tile::SlideNorth && a.1 == Direction::N
                || *grid.cell(a.0) == Tile::SlideEast && a.1 == Direction::E
                || *grid.cell(a.0) == Tile::SlideSouth && a.1 == Direction::S
                || *grid.cell(a.0) == Tile::SlideWest && a.1 == Direction::W
            {
                tiles.push(a.0);
            }
        }
        NextStep::Hub(tiles)
    } else {
        println!("==!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!1");
        NextStep::Error
    }
}

enum NextStep {
    Tile(Position),
    Hub(Vec<Position>),
    Finish,
    Error,
}

#[derive(Debug, Grid, PartialEq)]
enum Tile {
    #[symbol = '.']
    Path,
    #[symbol = '#']
    Forest,
    #[symbol = 'v']
    SlideSouth,
    #[symbol = '<']
    SlideWest,
    #[symbol = '>']
    SlideEast,
    #[symbol = '^']
    SlideNorth,
}
