fn main() {
    let grid: Grid<Cell> = Grid::from(include_str!("test.txt"));
    println!("{grid}");
}

#[derive(Debug)]
enum Cell {
    Round,
    Cube,
    Empty,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            'O' => Cell::Round,
            '#' => Cell::Cube,
            '.' => Cell::Empty,
            _ => unreachable!(),
        }
    }
}

impl From<&Cell> for char {
    fn from(value: &Cell) -> Self {
        match value {
            Cell::Round => 'O',
            Cell::Cube => '#',
            Cell::Empty => '.',
        }
    }
}

#[derive(Debug)]
struct Grid<T> {
    inner: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    fn new() -> Self {
        Grid { inner: Vec::new() }
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    char: for<'a> std::convert::From<&'a T>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.inner.iter() {
            writeln!(f, "{}", row.iter().map(char::from).collect::<String>())?
        }
        Ok(())
    }
}

impl<T> From<&str> for Grid<T>
where
    T: From<char>,
{
    fn from(value: &str) -> Self {
        value.lines().fold(Grid::new(), |mut grid, row| {
            grid.inner.push(row.chars().map(T::from).collect());
            grid
        })
    }
}
