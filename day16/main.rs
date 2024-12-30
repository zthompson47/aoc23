use aoc23::Grid;

fn main() {
    let grid = Grid::from(include_str!("test.txt"));
    println!("{grid}");
}

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    MirrorBack,
    MirrorForward,
    SplitterVertical,
    SplitterHorizontal,
    Empty,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Empty,
            '\\' => Cell::MirrorBack,
            '/' => Cell::MirrorForward,
            '|' => Cell::SplitterVertical,
            '-' => Cell::SplitterHorizontal,
            _ => unreachable!(),
        }
    }
}

impl From<&Cell> for char {
    fn from(value: &Cell) -> Self {
        match value {
            Cell::Empty => '.',
            Cell::MirrorBack => '\\',
            Cell::MirrorForward => '/',
            Cell::SplitterVertical => '|',
            Cell::SplitterHorizontal => '-',
        }
    }
}
