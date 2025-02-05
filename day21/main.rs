use std::{collections::HashSet, mem::swap};

use aoc23::Grid;
use grid_derive::Grid;

fn main() {
    let input = include_str!("input.txt");
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

    println!("Part 1: {}", positions.len());
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

/*
grid.print(|p, c| {
    if positions.contains(&p) {
        "O".to_string()
    } else {
        c.to_string()
    }
});
println!();
*/
