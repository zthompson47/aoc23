#![allow(unused)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    mem::swap,
};

use colored::Colorize;

use aoc23::{Alignment, Dimensions, Direction, Grid, Position};
use grid_derive::Grid;

fn main() {
    let test = include_str!("test.txt");
    let test1 = include_str!("test1.txt");
    let test2 = include_str!("test2.txt");
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input, 64));
    //println!("Part 1.01: {}", part1(input, 124));
    //println!("Part 1.01: {}", part1(input, 125));
    //println!("Part 1.01: {}", part1(input, 126));
    //println!("Part 1.01: {}", part1(input, 127));
    //println!("Part 1.01: {}", part1(input, 128));
    //println!("Part 1.01: {}", part1(input, 129));
    //println!("Part 1.01: {}", part1(input, 130));
    //println!("Part 1.01: {}", part1(input, 131));
    //println!("Part 1.01: {}", part1(input, 132));
    println!("Part 2: {}", part2());
    //println!("Part 2: {}", part2(input, 26501365));

    /*
    let factor = 15;
    let s = |x:usize| x * (factor / 2) + 1;
    //let s = |x:usize| x ;

    let mut grid = Grid::<Square>::from(test2);
    //let steps = grid.dim().r * (factor / 2);
    let steps = s(grid.dim().r);
    dbg!(steps);
    let start = grid.take_first(Square::Start).unwrap();
    let five_by_five = grid.square(factor);
    let start = Position::new(
        start.r + (factor / 2) * grid.dim().r,
        start.c + (factor / 2) * grid.dim().r,
    );
    let all_paths = paths(&five_by_five, start, steps);
    */

    /*
    let mut grid = Grid::<Square>::from(test2);
    //let steps = grid.dim().r * (factor / 2);
    let steps = s(grid.dim().r);
    dbg!(steps);
    let start = grid.take_first(Square::Start).unwrap();
    let mut grid_row = grid.clone();
    grid_row.extend(Alignment::Horizontal, grid.clone());
    grid_row.extend(Alignment::Horizontal, grid.clone());
    let start = Position::new(
        start.r,
        start.c + (factor / 2) * grid.dim().r,
    );
    let all_paths = paths(&grid_row, start, steps);

    let mut grid = Grid::<Square>::from(test1);
    //let steps = grid.dim().r * (factor / 2);
    let steps = s(grid.dim().r);
    let start = grid.take_first(Square::Start).unwrap();
    let five_by_five = grid.square(factor);
    let start = Position::new(
        start.r + (factor / 2) * grid.dim().r,
        start.c + (factor / 2) * grid.dim().r,
    );
    let all_paths = paths(&five_by_five, start, steps);

    let mut grid = Grid::<Square>::from(test2);
    let start = grid.take_first(Square::Start).unwrap();
    let grid_big = grid.square(7);
    let start = Position::new(start.r + 3 * grid.dim().r, start.c + 3 * grid.dim().c);
    let _ = paths(&grid_big, start, grid.dim().r * 3);
    */

    //println!("Fill grid: {}", part1(input, 131));

    //println!("Part 2: {}", part2(input, 1000));

    /*
    // (0, 0) 7521,7467
    // (1, 0) 7521,7467
    // (5, 0) 7521,7467
    // (0, 9) 7521,7467
    // 129 7467 (65, 65) 7521,7467
    // 259 7467 (0, 0) 7521,7467
    // 258 7467 (1, 0) 7521,7467
    // 257 7467 (2, 0) 7521,7467
    // 250 7467 (0, 121) 7521,7467
    let input = include_str!("input.txt");
    let start = Position::new(0, 121);
    for i in 0..6400 {
        let part0 = part0(input, start, i);
        println!("{i} {}", part0);
        if [7521, 7467].contains(&part0) {
            break;
        }
    }
    */

    //part00();
}

fn part2_notta_workin(input: &str, steps: usize) -> u64 {
    let mut grid = Grid::<Square>::from(input);
    let start = grid.take_first(Square::Start).unwrap();

    // Start is dead center of square grid.
    assert_eq!(grid.dim().r, grid.dim().c);
    assert_eq!(grid.dim().r - 1 - start.r, start.r);
    assert_eq!(grid.dim().c - 1 - start.c, start.c);

    let grid_lengths: u64 = steps as u64 / grid.dim().r as u64;
    let extra_steps = steps % grid.dim().r;
    let even_number = steps % 2 == 0;

    /*
    let paths_per_grid: u64 = paths(
        &grid,
        start,
        if even_number {
            grid.dim().r + 1
        } else {
            grid.dim().r
        },
    )
    .len() as u64;
    */

    let paths_per_grid = saturate(&grid, start, even_number).len() as u64;
    let seven_by_seven = grid.square(7);

    dbg!(seven_by_seven.dim());

    let seven_by_seven = paths(
        &seven_by_seven,
        Position::new(start.r + grid.dim().r * 3, start.c + grid.dim().c * 3),
        grid.dim().r * 2 + extra_steps,
    );

    dbg!(grid.dim());
    dbg!(steps);
    dbg!(even_number);
    dbg!(grid_lengths);
    dbg!(extra_steps);
    dbg!(even_number);
    dbg!(paths_per_grid);
    dbg!(seven_by_seven.len());

    let mut edge_small = 0;
    let mut edge_large = 0;
    let mut tips = 0;
    let Dimensions { r: g_r, c: g_c } = grid.dim();
    for Position { r, c } in seven_by_seven {
        let center_column = c > g_c * 3 && c < g_c * 4;
        let center_row = r > g_r * 3 && r < g_r * 4;
        let top_two = r > 0 && r < g_r * 2;
        let bottom_two = r > g_r * 5 && r < g_r * 7;
        let left_two = c > 0 && c < g_c * 2;
        let right_two = c > g_c * 5 && c < g_c * 7;
        let left_of_center = c > g_c * 2 && c < g_c * 3;
        let right_of_center = c > g_c * 4 && c < g_c * 5;
        let two_left_of_center = c > g_c && c < g_c * 2;
        let two_right_of_center = c > g_c * 5 && c < g_c * 6;
        let above_center = r > g_r * 2 && r < g_r * 3;
        let below_center = r > g_r * 4 && r < g_r * 5;
        let two_above_center = r > g_r && r < g_r * 2;
        let two_below_center = r > g_r * 5 && r < g_r * 6;
        if center_column && (top_two || bottom_two) {
            tips += 1;
        }
        if center_row && (left_two || right_two) {
            tips += 1;
        }
        if (two_above_center || two_below_center) && (left_of_center || right_of_center) {
            edge_small += 1;
        }
        if (above_center || below_center) && (two_left_of_center || two_right_of_center) {
            edge_large += 1;
        }
    }
    dbg!(tips);
    dbg!(edge_small);
    dbg!(edge_large);

    let full_grids: u64 = grid_lengths * grid_lengths * 2 - (grid_lengths + grid_lengths - 1);
    let mut result = full_grids * paths_per_grid;
    dbg!(full_grids);
    dbg!(result);
    result += tips;
    result += (grid_lengths - 1) * edge_small;
    result += (grid_lengths - 2) * edge_large;

    result

    /*
    let mut extended_grid_horizontal = grid.clone();
    extended_grid_horizontal.extend(Alignment::Horizontal, grid.clone());
    let mut extended_grid_vertical = grid.clone();
    extended_grid_vertical.extend(Alignment::Vertical, grid.clone());
    println!("=============== north ====================");
    let extra_north = paths(
        &extended_grid_vertical,
        Position::new(extended_grid_vertical.dim().r - 1, start.c),
        start.r + extra_steps,
    );
    println!("=============== east ====================");
    let extra_east = paths(
        &extended_grid_horizontal,
        Position::new(start.r, 0),
        start.c + extra_steps,
    );
    println!("=============== south ====================");
    let extra_south = paths(
        &extended_grid_vertical,
        Position::new(0, start.c),
        start.r + extra_steps,
    );
    println!("=============== west ====================");
    let extra_west = paths(
        &extended_grid_horizontal,
        Position::new(start.r, extended_grid_horizontal.dim().c - 1),
        start.c + extra_steps,
    );

    println!("=============== northeast ====================");
    let extra_northeast = paths(&grid, Position::new(grid.dim().r - 1, 0), extra_steps);
    let extra_northeast_big = paths(
        &grid,
        Position::new(grid.dim().r - 1, 0),
        extra_steps + grid.dim().r,
    );
    println!("=============== southeast ====================");
    let extra_southeast = paths(&grid, Position::new(0, 0), extra_steps);
    let extra_southeast_big = paths(&grid, Position::new(0, 0), extra_steps + grid.dim().r);
    println!("=============== southwest ====================");
    let extra_southwest = paths(&grid, Position::new(0, grid.dim().c - 1), extra_steps);
    let extra_southwest_big = paths(
        &grid,
        Position::new(0, grid.dim().c - 1),
        extra_steps + grid.dim().r,
    );
    println!("=============== northwest ====================");
    let extra_northwest = paths(
        &grid,
        Position::new(grid.dim().r - 1, grid.dim().c - 1),
        extra_steps,
    );
    let extra_northwest_big = paths(
        &grid,
        Position::new(grid.dim().r - 1, grid.dim().c - 1),
        extra_steps + grid.dim().r,
    );

    dbg!(grid_lengths, extra_steps, even_number, paths_per_grid);
    dbg!(&extra_east, &extra_west, &extra_north, &extra_south);
    dbg!(
        &extra_northeast,
        &extra_northeast_big,
        &extra_northwest,
        &extra_northwest_big,
        &extra_southeast,
        &extra_southeast_big,
        &extra_southwest,
        &extra_southwest_big,
    );

    if grid_lengths == 0 {
        let mut horizontal = grid.clone();
        horizontal.extend(Alignment::Horizontal, grid.clone());
        horizontal.extend(Alignment::Horizontal, grid.clone());
        let mut vertical = horizontal.clone();
        vertical.extend(Alignment::Vertical, horizontal.clone());
        vertical.extend(Alignment::Vertical, horizontal.clone());
        let start = Position::new(start.r + grid.dim().r, start.c + grid.dim().c);
        paths(&vertical, start, steps).len() as u64
    } else {
        let full_grids: u64 = grid_lengths * grid_lengths * 2 - (grid_lengths + grid_lengths - 1);
        let mut result = full_grids * paths_per_grid;
        result += extra_north.len() as u64
            + extra_east.len() as u64
            + extra_south.len() as u64
            + extra_west.len() as u64;

        result += (grid_lengths - 1) * extra_northeast.len() as u64
            + (grid_lengths - 1) * extra_southeast.len() as u64
            + (grid_lengths - 1) * extra_southwest.len() as u64
            + (grid_lengths - 1) * extra_northwest.len() as u64;

        if grid_lengths > 1 {
            result += (grid_lengths - 2) * extra_northeast_big.len() as u64
                + (grid_lengths - 2) * extra_southeast_big.len() as u64
                + (grid_lengths - 2) * extra_southwest_big.len() as u64
                + (grid_lengths - 2) * extra_northwest_big.len() as u64;
        }

        /*
        result += grid_lengths * extra_northeast as u64
            + grid_lengths * extra_southeast as u64
            + grid_lengths * extra_southwest as u64
            + grid_lengths * extra_northwest as u64;
            */

        // 611176059429704 is too low..
        // 611176753109022 is too low..
        // 615596662580676 is to high..
        result
    }
    */
}

struct GridEvolver<T> {
    growing_counts: Vec<T>,
    even_odd_counts: [T; 2],
}

impl<T> GridEvolver<T> {
    fn new(grid: &Grid<Square>, start: Position, extra_steps: usize) -> Self {
        todo!()
    }
}

impl<T> Iterator for GridEvolver<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn part00() {
    let input = include_str!("input.txt");
    let mut grid = Grid::<Square>::from(input);
    let start = grid.take_first(Square::Start).unwrap();
    let start = Position::new(130, 0);
    let mut positions = HashSet::from([start]);
    let mut new_positions = HashSet::new();
    let mut i = 0;
    loop {
        for position in positions.drain() {
            new_positions.extend(position.adjacent_if(&grid, |square| *square == Square::Garden));
        }
        swap(&mut positions, &mut new_positions);

        grid.print(|p, c| {
            if positions.contains(&p) {
                "O".to_string()
            } else {
                c.to_string()
            }
        });
        println!();

        if positions
            .iter()
            //.any(|p| p.r == 0 || p.c == 0 || p.r == 131 || p.c == 131)
            //.any(|p| p.r == 0 || p.r == 131 || p.c == 131)
            //.any(|p| p.c == 130)
            .any(|p| p.c == 65)
        {
            break;
        }

        if positions.len() == 7467 {
            i += 1;
            if i > 3 {
                break;
            }
        }
    }
}

fn part0(input: &str, start: Position, steps: usize) -> usize {
    let mut grid = Grid::<Square>::from(input);
    let start = grid.take_first(Square::Start).unwrap();
    //println!("{_start:?}");
    //println!("{:?}", grid.dim());
    //let mut positions = HashSet::from([start, Position::new(start.r, start.c + 1)]);
    let mut positions = HashSet::from([start]);
    let mut new_positions = HashSet::new();
    for _ in 0..steps {
        for position in positions.drain() {
            new_positions.extend(position.adjacent_if(&grid, |square| *square == Square::Garden));
        }
        swap(&mut positions, &mut new_positions);
    }

    grid.print(|p, c| {
        if positions.contains(&p) {
            "O".to_string()
        } else {
            c.to_string()
        }
    });

    positions.len()
}

fn part1(input: &str, steps: usize) -> usize {
    let mut grid = Grid::<Square>::from(input);
    /*println!(
        "empty cells: {}",
        grid.cells().filter(|x| **x == Square::Garden).count()
    );*/
    let start = grid.take_first(Square::Start).unwrap();
    let path_fn = paths_saturate(&grid, start);

    path_fn(steps)
}

fn part2() -> usize {
    let steps = 26501365;

    let mut grid = Grid::<Square>::from(include_str!("input.txt"));
    assert_eq!(grid.dim(), Dimensions::new(131, 131));

    let start = grid.take_first(Square::Start).unwrap();
    assert_eq!(start, Position::new(65, 65));

    let start_n = Position::new(130, 65);
    let start_e = Position::new(65, 0);
    let start_s = Position::new(0, 65);
    let start_w = Position::new(65, 130);

    let start_ne = Position::new(130, 0);
    let start_se = Position::new(0, 0);
    let start_sw = Position::new(0, 130);
    let start_nw = Position::new(130, 130);

    let middle = paths_saturate(&grid, start);

    let grid_n = paths_saturate(&grid, start_n);
    let grid_e = paths_saturate(&grid, start_e);
    let grid_s = paths_saturate(&grid, start_s);
    let grid_w = paths_saturate(&grid, start_w);

    let grid_ne = paths_saturate(&grid, start_ne);
    let grid_se = paths_saturate(&grid, start_se);
    let grid_sw = paths_saturate(&grid, start_sw);
    let grid_nw = paths_saturate(&grid, start_nw);

    let mut result = middle(steps);

    let mut i = steps - 66;
    loop {
        result += grid_n(i) + grid_e(i) + grid_s(i) + grid_w(i);
        if i < 131 {
            break;
        }
        i -= 131;
    }

    let mut i = steps - 132;
    let mut j = 1;
    loop {
        result += j * (grid_ne(i) + grid_se(i) + grid_sw(i) + grid_nw(i));
        if i < 131 {
            break;
        }
        i -= 131;
        j += 1;
    }

    result
}

//fn paths(grid: &Grid<Square>, start: Position, steps: usize) -> usize {
fn paths(grid: &Grid<Square>, start: Position, steps: usize) -> HashSet<Position> {
    let mut positions = HashSet::from([start]);
    let mut new_positions = HashSet::new();
    for count in 0..steps {
        /*
        grid.print(|p, c| {
            if positions.contains(&p) {
                "O".red()
            } else {
                c.to_string().into()
            }
        });
        println!();
        */

        //        let asdfasdf = positions.clone();

        for position in positions.drain() {
            new_positions.extend(position.adjacent_if(grid, |square| *square == Square::Garden));
        }

        //        grid.print(|p, c| {
        //           if new_positions.contains(&p) || asdfasdf.contains(&p) {
        //              "O".red()
        //         } else {
        //            c.to_string().into()
        //       }
        //  });
        // println!();

        swap(&mut positions, &mut new_positions);
        println!("{count}, {}", positions.len());
    }

    /*
    grid.print(|p, c| {
        if positions.contains(&p) {
            "O".red()
        } else {
            c.to_string().into()
        }
    });
    println!();
    */

    //positions.len()
    positions
}

fn paths_saturate(grid: &Grid<Square>, start: Position) -> impl Fn(usize) -> usize {
    let mut positions = HashSet::from([start]);
    let mut new_positions = HashSet::new();
    let mut results: Vec<usize> = Vec::from([1]);

    for idx in 0.. {
        for position in positions.drain() {
            new_positions.extend(position.adjacent_if(grid, |square| *square == Square::Garden));
        }

        /*grid.print(|p, c| {
            if new_positions.contains(&p) {
                "O".red()
            } else {
                c.to_string().into()
            }
        });
        println!();*/

        //println!("{idx}, {}, ", new_positions.len());

        // Check if all paths have been found when total count is cycling between two numbers.
        if idx > 1 && results[idx - 1] == new_positions.len() {
            break;
        }

        results.push(new_positions.len());
        swap(&mut positions, &mut new_positions);
    }

    move |x| {
        if x < results.len() {
            results[x]
        } else if results.len() % 2 == x % 2 {
            results[results.len() - 2]
        } else {
            results[results.len() - 1]
        }
    }
}

fn saturate(grid: &Grid<Square>, start: Position, even: bool) -> HashSet<Position> {
    let mut positions = HashSet::from([start]);
    let mut new_positions = HashSet::new();
    let mut iterations = 0;
    let mut latest_path_counts: VecDeque<u64> = VecDeque::new();

    loop {
        iterations += 1;
        let is_even = iterations % 2 == 0;
        for position in positions.drain() {
            new_positions.extend(position.adjacent_if(grid, |square| *square == Square::Garden));
        }
        swap(&mut positions, &mut new_positions);
        println!("----> {}", positions.len());

        if latest_path_counts.len() == 2 {
            let two_ago = latest_path_counts.pop_front().unwrap();
            if is_even == even && positions.len() as u64 == two_ago {
                println!("_______>>  ITERATTIONS: {}", iterations);
                break positions;
            }
        }

        latest_path_counts.push_back(positions.len() as u64);
    }
}

fn part2_old(input: &str, steps: usize) -> u128 {
    let mut grid = Grid::<Square>::from(input);
    let start = grid.take_first(Square::Start).unwrap();

    type InfiniteGrid = HashMap<Position, HashSet<SignedPosition>>;

    let mut paths: InfiniteGrid =
        HashMap::from([(start, HashSet::from([SignedPosition::new(0, 0)]))]);
    let mut new_paths: InfiniteGrid = HashMap::new();

    for _ in 0..steps {
        for (position, layers) in paths.drain() {
            for (new_position, direction) in
                position.adjacent_if_wrapping(&grid, |square| *square == Square::Garden)
            {
                let new_layers = if let Some(direction) = direction {
                    &layers.iter().map(|x| x.step(direction)).collect()
                } else {
                    &layers
                };
                new_paths
                    .entry(new_position)
                    .and_modify(|old_layers| old_layers.extend(new_layers.clone()))
                    .or_insert(new_layers.clone());
            }
        }
        swap(&mut paths, &mut new_paths);
    }

    paths.values().map(|x| x.len() as u128).sum::<u128>()
}

#[derive(Clone, Copy, Default, PartialEq, Grid)]
enum Square {
    #[default]
    #[symbol = '.']
    Garden,
    #[symbol = '#']
    Rock,
    #[symbol = 'S']
    Start,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SignedPosition {
    pub r: isize,
    pub c: isize,
}

impl SignedPosition {
    pub fn new(r: isize, c: isize) -> Self {
        SignedPosition { r, c }
    }

    pub fn step(&self, direction: Direction) -> Self {
        match direction {
            Direction::N => SignedPosition::new(self.r - 1, self.c),
            Direction::E => SignedPosition::new(self.r, self.c + 1),
            Direction::S => SignedPosition::new(self.r + 1, self.c),
            Direction::W => SignedPosition::new(self.r, self.c - 1),
        }
    }
}

#[test]
fn signed_position() {
    let position = SignedPosition::new(0, 0);
    assert_eq!(SignedPosition::new(-1, 0), position.step(Direction::N));
    assert_eq!(SignedPosition::new(0, 1), position.step(Direction::E));
    assert_eq!(SignedPosition::new(1, 0), position.step(Direction::S));
    assert_eq!(SignedPosition::new(0, -1), position.step(Direction::W));
}
