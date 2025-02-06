use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

use aoc23::{Direction, Grid, Position};
use grid_derive::Grid;

fn main() {
    let input = include_str!("test.txt");
    //println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input, 1000));
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::<Square>::from(input);
    let start = grid.take_first(Square::Start).unwrap();

    let mut positions = HashSet::from([start]);
    let mut new_positions = HashSet::new();
    for _ in 0..64 {
        for position in positions.drain() {
            new_positions.extend(position.adjacent_if(&grid, |square| *square == Square::Garden));
        }
        swap(&mut positions, &mut new_positions);
    }

    positions.len()
}

fn part2(input: &str, steps: usize) -> u128 {
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

#[derive(Default, PartialEq, Grid)]
enum Square {
    #[default]
    #[symbol = '.']
    Garden,
    #[symbol = '#']
    Rock,
    #[symbol = 'S']
    Start,
}

#[test]
fn short_walks() {
    let input = include_str!("test.txt");
    assert_eq!(part2(input, 6), 16);
    assert_eq!(part2(input, 10), 50);
    assert_eq!(part2(input, 50), 1594);
    assert_eq!(part2(input, 100), 6536);
    assert_eq!(part2(input, 500), 167004);
    assert_eq!(part2(input, 1000), 668697);
    assert_eq!(part2(input, 5000), 16733044);
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

/*
grid.print(|p, c| {
    if new_paths.keys().any(|pp| *pp == p) {
        "O".to_string()
    } else {
        c.to_string()
    }
});
*/
