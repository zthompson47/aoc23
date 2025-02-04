use std::collections::HashSet;

use aoc23::Grid;
use grid_derive::Grid;

fn main() {
    let input = include_str!("input.txt");
    let mut grid = Grid::<Square>::from(input);
    println!("{grid}\n");

    let start = grid.swap_first(Square::Start, Square::Garden).unwrap();
    let mut positions = HashSet::from([start]);
    for _ in 0..64 {
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
        let mut new_positions = HashSet::new();
        for position in positions.iter() {
            new_positions.extend(position.adjacent_if(&grid, |square| *square == Square::Garden));
        }
        positions = new_positions;
    }

    println!("Part 1: {}", positions.len());
}

#[derive(Debug, PartialEq, Clone, Copy, Grid)]
enum Square {
    #[symbol = '.']
    Garden,
    #[symbol = '#']
    Rock,
    #[symbol = 'S']
    Start,
}
