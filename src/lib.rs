#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T>
where
    T: Clone,
{
    inner: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Grid { inner: Vec::new() }
    }

    pub fn dimensions(&self) -> Dimensions {
        Dimensions {
            r: self.inner.len(),
            c: self.inner[0].len(),
        }
    }
}

pub struct Dimensions {
    pub r: usize,
    pub c: usize,
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

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Clone)]
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
