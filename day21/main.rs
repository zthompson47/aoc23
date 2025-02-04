use aoc23::Grid;
use grid_derive::Grid;

fn main() {
    let input = include_str!("test.txt");
    let mut grid = Grid::<Square>::from(input);
    println!("{grid}\n");
    let start = grid.swap_first(Square::Start, Square::Garden).unwrap();
    println!("{grid}");
    println!("Start:{start:?}");
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
