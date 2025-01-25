#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T>
where
    T: Clone,
{
    pub inner: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Grid { inner: Vec::new() }
    }

    pub fn dim(&self) -> Dimensions {
        Dimensions {
            r: self.inner.len(),
            c: self.inner[0].len(),
        }
    }

    pub fn cell(&self, position: Position) -> &T {
        &self.inner[position.r][position.c]
    }

    pub fn cell_mut(&mut self, position: Position) -> &mut T {
        &mut self.inner[position.r][position.c]
    }

    pub fn cells(&self) -> impl Iterator<Item = &T> {
        self.inner.iter().flatten()
    }

    pub fn bottom_right(&self) -> Position {
        Position {
            r: self.dim().r - 1,
            c: self.dim().c - 1,
        }
    }

    pub fn top_left(&self) -> Position {
        Position { r: 0, c: 0 }
    }

    pub fn print<R>(&self, map: impl Fn(Position, char) -> R)
    where
        R: std::fmt::Display,
        char: for<'a> std::convert::From<&'a T>,
    {
        for r in 0..self.dim().r {
            for c in 0..self.dim().c {
                let p = Position::new(r, c);
                print!("{}", map(p, char::from(self.cell(p))));
            }
            println!()
        }
    }

    pub fn diagonals(&self) -> impl Iterator<Item = Vec<Position>> {
        assert_eq!(self.dim().r, self.dim().c);
        let size = self.dim().r;

        let bottom_right = (1..size).map(move |i| {
            let mut result = Vec::new();
            for (i_row, i_column) in (size - i..size).rev().zip(size - i..size) {
                let start = Position::new(i_row, i_column);
                result.push(start);
            }
            result
        });

        let middle: Vec<Vec<Position>> = vec![(0..size)
            .zip((0..size).rev())
            .map(|(r, c)| Position { r, c })
            .collect()];

        let top_left = (1..size).rev().map(|i| {
            let mut result = Vec::new();
            for (i_row, i_column) in (0..i).zip((0..i).rev()) {
                let start = Position::new(i_row, i_column);
                result.push(start);
            }
            result
        });

        bottom_right.chain(middle).chain(top_left)
    }
}

impl<T> From<&str> for Grid<T>
where
    T: From<char>,
    T: Clone,
{
    fn from(value: &str) -> Self {
        value.lines().fold(Grid::new(), |mut grid, row| {
            grid.inner.push(row.chars().map(T::from).collect());
            grid
        })
    }
}

impl<T> From<Dimensions> for Grid<T>
where
    T: Clone + Default,
{
    fn from(dimensions: Dimensions) -> Self {
        Self {
            inner: vec![vec![T::default(); dimensions.c]; dimensions.r],
        }
    }
}

impl<T> std::fmt::Display for Grid<T>
where
    char: for<'a> std::convert::From<&'a T>,
    T: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.inner
                .iter()
                .map(|r| r.iter().map(char::from).collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub r: usize,
    pub c: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:2}, {:2})", self.r, self.c)
    }
}

impl Position {
    pub const ORIGIN: Position = Position { r: 0, c: 0 };

    pub fn new(r: usize, c: usize) -> Self {
        Position { r, c }
    }

    pub fn adjacent<T>(&self, grid: &Grid<T>) -> Vec<Self>
    where
        T: Clone,
    {
        let mut result = vec![];
        for direction in Direction::all() {
            if let Some(step) = self.step(direction, grid) {
                result.push(step);
            }
        }
        result
    }

    pub fn steps<T>(&self, count: usize, direction: Direction, grid: &Grid<T>) -> Vec<Self>
    where
        T: Clone,
    {
        let mut result = Vec::new();
        let mut start = *self;
        for _ in 0..count {
            if let Some(position) = start.step(direction, grid) {
                result.push(position);
                start = position;
            } else {
                break;
            }
        }
        result
    }

    pub fn step<T>(&self, direction: Direction, grid: &Grid<T>) -> Option<Self>
    where
        T: Clone,
    {
        match direction {
            Direction::N => {
                if self.r > 0 {
                    Some(Position {
                        r: self.r - 1,
                        c: self.c,
                    })
                } else {
                    None
                }
            }
            Direction::E => {
                if self.c < grid.dim().c - 1 {
                    Some(Position {
                        r: self.r,
                        c: self.c + 1,
                    })
                } else {
                    None
                }
            }
            Direction::S => {
                if self.r < grid.dim().r - 1 {
                    Some(Position {
                        r: self.r + 1,
                        c: self.c,
                    })
                } else {
                    None
                }
            }
            Direction::W => {
                if self.c > 0 {
                    Some(Position {
                        r: self.r,
                        c: self.c - 1,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dimensions {
    pub r: usize,
    pub c: usize,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" => Direction::E,
            "L" => Direction::W,
            "U" => Direction::N,
            "D" => Direction::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Alignment {
    Horizontal,
    Vertical,
}

impl Alignment {
    pub fn all() -> [Self; 2] {
        [Alignment::Horizontal, Alignment::Vertical]
    }

    pub fn directions(&self) -> [Direction; 2] {
        match self {
            Alignment::Horizontal => [Direction::E, Direction::W],
            Alignment::Vertical => [Direction::S, Direction::N],
        }
    }

    pub fn orthogonal(&self) -> Self {
        match self {
            Alignment::Horizontal => Alignment::Vertical,
            Alignment::Vertical => Alignment::Horizontal,
        }
    }
}

impl Direction {
    pub fn all() -> [Self; 4] {
        [Direction::N, Direction::E, Direction::S, Direction::W]
    }

    pub fn opposite(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }

    pub fn orthogonal(&self) -> [Self; 2] {
        match self {
            Direction::N | Direction::S => [Direction::E, Direction::W],
            Direction::E | Direction::W => [Direction::S, Direction::N],
        }
    }

    pub fn alignment(&self) -> Alignment {
        match self {
            Direction::N | Direction::S => Alignment::Vertical,
            Direction::E | Direction::W => Alignment::Horizontal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone, Copy)]
    enum Cell {
        Full,
        Empty,
    }

    impl From<char> for Cell {
        fn from(value: char) -> Self {
            match value {
                '.' => Cell::Empty,
                '#' => Cell::Full,
                _ => unreachable!(),
            }
        }
    }

    impl From<&Cell> for char {
        fn from(value: &Cell) -> Self {
            match value {
                Cell::Empty => '.',
                Cell::Full => '#',
            }
        }
    }

    #[test]
    fn grid_load_and_display() {
        let input = "..#.\n..#.\n....\n...#\n";
        let grid = Grid::<Cell>::from(input);
        assert_eq!(input.trim(), format!("{grid}"));
    }
}
