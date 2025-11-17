use std::collections::{BTreeSet, HashMap, HashSet};

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
    lp(&grid, start, Weather::Slippery, &mut cache, visited, None).unwrap()
}

fn part2(input: &str) -> usize {
    let grid = Grid::<Tile>::from(input);
    let start = Position::new(0, 1);
    let visited: BTreeSet<Position> = BTreeSet::new();
    let mut cache = HashMap::new();
    lp(
        &grid,
        start,
        Weather::NotSlippery,
        &mut cache,
        visited,
        None,
    )
    .unwrap()
}

fn lp(
    grid: &Grid<Tile>,
    start: Position,
    weather: Weather,
    cache: &mut HashMap<Position, (usize, Position)>,
    mut visited: BTreeSet<Position>,
    mut prior_tile: Option<Position>,
) -> Option<usize> {
    //println!("lp {start}");

    let mut steps = 0;
    let mut current_tile = start;
    let mut current_prior_tile = prior_tile;

    loop {
        if let Some((distance, destination)) = cache.get(&current_tile) {
            //println!("   {distance} GOT CACHE {current_tile} to {destination}");
            //println!(" VISITED {visited:?}");
            steps += distance;
            current_tile = *destination;
            current_prior_tile = None;
        }
        match identify_tile(grid, current_tile, current_prior_tile, &visited, weather) {
            TileType::Finish => {
                steps += 1;
                //println!("{steps} FINISH {current_prior_tile:?}");
                break;
            }
            TileType::Hub(tiles) => {
                cache.insert(start, (steps, current_tile));
                if let (Some(hub_entrance), Some(prior_hub)) = (current_prior_tile, prior_tile) {
                    cache.insert(hub_entrance, (steps, prior_hub));
                }

                steps += 1;
                if !visited.insert(current_tile) {
                    //println!("---------> LOOP at {current_tile}");
                    return None;
                }
                //println!("{steps} hub {current_tile} to {tiles:?}");

                let mut max_steps = 0;
                for tile in tiles {
                    if let Some(distance) = lp(
                        grid,
                        tile,
                        weather,
                        cache,
                        visited.clone(),
                        Some(current_tile),
                    ) {
                        max_steps = max_steps.max(distance);
                    }
                    //break;
                }
                steps += max_steps;
                break;
            }
            TileType::Step(position) => {
                if !is_start(grid, current_tile) {
                    steps += 1;
                }
                current_prior_tile = Some(current_tile);
                current_tile = position;
                //println!("step from {current_prior_tile:?} to {current_tile}");
            }
            TileType::Error => panic!(),
        }
    }

    //println!("{steps} fp returns {start} {current_tile}");
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

fn is_start(grid: &Grid<Tile>, tile: Position) -> bool {
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
    //println!("next_step prior{prior_tile:?} to {tile}");
    let mut adjacent = tile.adjacent_if_direction(grid, |t, _| *t != Tile::Forest);

    if let Some(prior) = prior_tile {
        adjacent.retain(|x| x.0 != prior);
    }

    if adjacent.len() == 1 {
        Some(adjacent[0].0)
    } else {
        //println!("adjacent > 1 : len {}", adjacent.len());
        None
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Weather {
    Slippery,
    NotSlippery,
}

fn longest_path(
    grid: &Grid<Tile>,
    start: Position,
    weather: Weather,
    cache: &mut HashMap<Position, (usize, Position, Option<Position>)>,
    mut visited: BTreeSet<Position>,
    mut prior_tile: Option<Position>,
) -> Option<usize> {
    //println!("----> longest_path {start}");

    let mut steps = 0;
    let mut current_tile = start;
    println!("longest_path {start}");
    if !visited.insert(start) {
        return None;
    }

    loop {
        if current_tile == Position::new(1, 1) {
            println!("\n!!!!!!!!!!!!!!!!!!!WW{current_tile}WWW -- {prior_tile:?}");
            println!("  --> {visited:?}\n");
        }
        if let Some((distance, destination, prior)) = cache.get(&current_tile) {
            println!("GOT CACHE: {current_tile} to {destination} from {prior:?}");
            steps += distance;
            current_tile = *destination;
            prior_tile = *prior;
        }
        match _next_step(grid, current_tile, weather, prior_tile, visited.clone()) {
            Step::Tile(position) => {
                steps += 1;
                prior_tile = Some(current_tile);
                current_tile = position;
            }
            Step::Finish => {
                println!("FINISH {:?}", visited);
                steps += 1;
                cache.insert(start, (steps, current_tile, prior_tile));
                break;
            }
            Step::Hub(positions) => {
                //println!("=> HUB {positions:?}");
                cache.insert(start, (steps, current_tile, prior_tile));
                let mut max_steps = 0;
                for position in positions {
                    if position.c == 3 {
                        println!(
                            "oooooooooooooooo{position}oooooprio{current_tile}ooooooooooooooooooo"
                        );
                    }
                    //visited.insert(current_tile); // JUST ADDED
                    if let Some(s) = longest_path(
                        grid,
                        position,
                        weather,
                        cache,
                        visited.clone(),
                        Some(current_tile),
                    ) {
                        max_steps = max_steps.max(s + 1);
                    }
                }
                steps += max_steps;
                break;
            }
            Step::Error => panic!(),
        }
    }

    Some(steps)
}

fn _next_step(
    grid: &Grid<Tile>,
    current_tile: Position,
    weather: Weather,
    prior_tile: Option<Position>,
    visited: BTreeSet<Position>,
) -> Step {
    println!("\nnext_step current: {current_tile} prior: {prior_tile:?}");
    let mut adjacent = current_tile.adjacent_if_direction(grid, |t, _| *t != Tile::Forest);
    if let Some(prior) = prior_tile {
        adjacent.retain(|x| x.0 != prior && !visited.contains(&x.0));
    }
    println!("adjacent: {adjacent:?}");

    if adjacent.is_empty() {
        println!("-------ERROR-----------");
        Step::Error
    } else if adjacent.len() == 1 {
        println!("adjacent len 1");
        if adjacent[0].0 == Position::new(grid.dim().r - 1, grid.dim().c - 2) {
            Step::Finish
        } else {
            Step::Tile(adjacent[0].0)
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
        println!("is hub");
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
        Step::Hub(tiles)
    } else {
        println!("-------ERROR-----------");
        Step::Error
    }
}

enum Step {
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

fn find_longest_path(
    grid: &Grid<Tile>,
    start: Position,
    mut visited: HashSet<Position>,
    slippery: bool,
    solutions: &mut Vec<usize>,
    cache: &mut HashMap<Position, (usize, Position)>,
) -> Option<usize> {
    println!("---------------------------------");
    let mut steps = 0;
    let mut current_tile = start;
    loop {
        if let Some((cached_steps, position)) = cache.get(&current_tile) {
            println!("GOT CACHE for {current_tile}: {cached_steps} {position}");
            steps += cached_steps;
            current_tile = *position;
        }
        if !visited.insert(current_tile) {
            println!("???????????{current_tile}?????????????????????????????????");
            return None;
        }
        match find_next_step(grid, current_tile, &visited, slippery) {
            NextStep::Tile(next_tile) => {
                current_tile = next_tile;
                steps += 1;
            }
            NextStep::Slope(next_tile) => {
                current_tile = next_tile;
                steps += 1;
                //println!("CACHE is: {cache:?}");
                //panic!();
            }
            NextStep::Hub(positions) => {
                // Choose longest branch.
                steps += 1;
                cache.insert(start, (steps, current_tile));
                let mut max_steps = 0;
                for position in positions {
                    if let Some(steps) = find_longest_path(
                        grid,
                        position,
                        visited.clone(),
                        slippery,
                        solutions,
                        cache,
                    ) {
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
                //steps += 1;
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
    println!("ADJACENT after: {} {adjacent:?}", adjacent.len());

    if adjacent.len() == 1 {
        if adjacent[0].0 == Position::new(grid.dim().r - 1, grid.dim().c - 2) {
            NextStep::Finish
        } else if [
            Tile::SlideSouth,
            Tile::SlideWest,
            Tile::SlideEast,
            Tile::SlideNorth,
        ]
        .contains(grid.cell(adjacent[0].0))
        {
            NextStep::Slope(adjacent[0].0)
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
        println!("00000");
        let mut tiles = vec![];
        for a in adjacent {
            if !slippery
                || *grid.cell(a.0) == Tile::SlideNorth && a.1 == Direction::N
                || *grid.cell(a.0) == Tile::SlideEast && a.1 == Direction::E
                || *grid.cell(a.0) == Tile::SlideSouth && a.1 == Direction::S
                || *grid.cell(a.0) == Tile::SlideWest && a.1 == Direction::W
            {
                println!("11111");
                tiles.push(a.0);
            }
        }
        NextStep::Hub(tiles)
    } else {
        println!("==!!!!!!!!!!!!!{tile}!!!!{adjacent:?}!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!1");
        NextStep::Error
    }
}

enum NextStep {
    Tile(Position),
    Slope(Position),
    Hub(Vec<Position>),
    Finish,
    Error,
}

fn _part1(input: &str) -> usize {
    let grid = Grid::<Tile>::from(input);
    let start = Position::new(0, 1);
    let visited: HashSet<Position> = HashSet::new();
    let mut solutions = Vec::new();
    let mut cache = HashMap::new();
    find_longest_path(&grid, start, visited, true, &mut solutions, &mut cache).unwrap()
    //let old = find_longest_path(&grid, start, visited, true, &mut solutions).unwrap();
    //*solutions.iter().max().unwrap()
}

fn _part2(input: &str) -> usize {
    let grid = Grid::<Tile>::from(input);
    let start = Position::new(0, 1);
    let visited: HashSet<Position> = HashSet::new();
    let mut solutions = Vec::new();
    let mut cache = HashMap::new();
    let old = find_longest_path(&grid, start, visited, false, &mut solutions, &mut cache).unwrap();
    *solutions.iter().max().unwrap()
}
