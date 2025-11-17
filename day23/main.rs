use std::collections::{BTreeSet, HashMap};

use aoc23::{Direction, Grid, Position};
use grid_derive::Grid;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let grid = Grid::<Tile>::from(input);
    let start = Position::new(0, 1);
    let visited: BTreeSet<Position> = BTreeSet::new();
    let mut cache = HashMap::new();
    longest_path(&grid, start, Weather::Slippery, &mut cache, visited, None).unwrap()
}

fn part2(input: &str) -> usize {
    let grid = Grid::<Tile>::from(input);
    let start = Position::new(0, 1);
    let visited: BTreeSet<Position> = BTreeSet::new();
    let mut cache = HashMap::new();
    longest_path(
        &grid,
        start,
        Weather::NotSlippery,
        &mut cache,
        visited,
        None,
    )
    .unwrap()
}

fn longest_path(
    grid: &Grid<Tile>,
    start: Position,
    weather: Weather,
    cache: &mut HashMap<Position, (usize, Position)>,
    mut visited: BTreeSet<Position>,
    prior_tile: Option<Position>,
) -> Option<usize> {
    //println!("lp {start}");

    let mut steps = 0;
    let mut current_tile = start;
    let mut current_prior_tile = prior_tile;

    loop {
        if let Some((distance, destination)) = cache.get(&current_tile) {
            steps += distance;
            current_tile = *destination;
            current_prior_tile = None;
        }
        match identify_tile(grid, current_tile, current_prior_tile, &visited, weather) {
            TileType::Finish => {
                steps += 1;
                break;
            }
            TileType::Hub(tiles) => {
                cache.insert(start, (steps, current_tile));
                if let (Some(hub_entrance), Some(prior_hub)) = (current_prior_tile, prior_tile) {
                    cache.insert(hub_entrance, (steps, prior_hub));
                }

                steps += 1;
                if !visited.insert(current_tile) {
                    return None;
                }

                let mut max_steps = 0;
                for tile in tiles {
                    if let Some(distance) = longest_path(
                        grid,
                        tile,
                        weather,
                        cache,
                        visited.clone(),
                        Some(current_tile),
                    ) {
                        max_steps = max_steps.max(distance);
                    }
                }
                steps += max_steps;
                break;
            }
            TileType::Step(position) => {
                if !is_start(current_tile) {
                    steps += 1;
                }
                current_prior_tile = Some(current_tile);
                current_tile = position;
            }
            TileType::Error => panic!(),
        }
    }

    Some(steps)
}

fn identify_tile(
    grid: &Grid<Tile>,
    tile: Position,
    prior_tile: Option<Position>,
    visited: &BTreeSet<Position>,
    weather: Weather,
) -> TileType {
    if is_finish(grid, tile) {
        TileType::Finish
    } else if is_hub(grid, tile) {
        let mut adjacent = tile.adjacent_if_direction(grid, |t, _| *t != Tile::Forest);
        if let Some(prior) = prior_tile {
            adjacent.retain(|x| x.0 != prior && !visited.contains(&x.0));
        }

        let mut tiles = vec![];
        for a in adjacent {
            if weather == Weather::NotSlippery
                || *grid.cell(a.0) == Tile::SlideNorth && a.1 == Direction::N
                || *grid.cell(a.0) == Tile::SlideEast && a.1 == Direction::E
                || *grid.cell(a.0) == Tile::SlideSouth && a.1 == Direction::S
                || *grid.cell(a.0) == Tile::SlideWest && a.1 == Direction::W
            {
                tiles.push(a.0);
            }
        }
        TileType::Hub(tiles)
    } else if let Some(next_step) = next_step(grid, tile, prior_tile) {
        TileType::Step(next_step)
    } else {
        TileType::Error
    }
}

enum TileType {
    Finish,
    Hub(Vec<Position>),
    Step(Position),
    Error,
}

fn is_finish(grid: &Grid<Tile>, tile: Position) -> bool {
    tile == Position::new(grid.dim().r - 1, grid.dim().c - 2)
}

fn is_start(tile: Position) -> bool {
    tile == Position::new(0, 1)
}

fn is_hub(grid: &Grid<Tile>, tile: Position) -> bool {
    let adjacent = tile.adjacent_if_direction(grid, |t, _| *t != Tile::Forest);
    !adjacent.is_empty()
        && adjacent.iter().all(|x| {
            [
                Tile::SlideSouth,
                Tile::SlideWest,
                Tile::SlideEast,
                Tile::SlideNorth,
            ]
            .contains(grid.cell(x.0))
        })
}

fn next_step(grid: &Grid<Tile>, tile: Position, prior_tile: Option<Position>) -> Option<Position> {
    let mut adjacent = tile.adjacent_if_direction(grid, |t, _| *t != Tile::Forest);

    if let Some(prior) = prior_tile {
        adjacent.retain(|x| x.0 != prior);
    }

    if adjacent.len() == 1 {
        Some(adjacent[0].0)
    } else {
        None
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Weather {
    Slippery,
    NotSlippery,
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
