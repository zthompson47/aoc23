use std::collections::HashMap;

fn main() {
    let mut grid: Grid<Cell> = Grid::from(include_str!("input.txt"));
    println!("{grid}");
    grid.tilt(Direction::N);
    println!("{grid}");

    println!("Part 1: {}", grid.score());
}

impl Grid<Cell> {
    fn score(&self) -> usize {
        let mut score = 0;
        for (i, row) in self.inner.iter().enumerate() {
            let rank = self.dimensions().r - i;
            score += row.iter().filter(|x| **x == Cell::Round).count() * rank;
        }
        score
    }

    fn tilt(&mut self, direction: Direction) {
        if [Direction::N, Direction::S].contains(&direction) {
            for column in 0..self.dimensions().c {
                self.tilt_column(column, direction);
            }
        } else {
            for column in 0..self.dimensions().r {
                self.tilt_row(column, direction);
            }
        }
    }

    fn tilt_column(&mut self, column: usize, direction: Direction) {
        // Scan for round rocks and store pile-up points at cube rocks.
        let mut cubes: HashMap<usize, usize> = HashMap::new();
        let mut rounds = 0;
        for mut row in 0..self.dimensions().r {
            row = match direction {
                Direction::N => self.dimensions().r - row - 1,
                Direction::S => todo!(),
                _ => unreachable!(),
            };
            match self.inner[row][column] {
                Cell::Round => {
                    rounds += 1;
                    self.inner[row][column] = Cell::Empty;
                }
                Cell::Cube => {
                    if rounds > 0 {
                        cubes.insert(row + 1, rounds);
                        rounds = 0;
                    }
                }
                Cell::Empty => {}
            }
        }
        match direction {
            Direction::N => cubes.insert(0, rounds),
            Direction::S => todo!(),
            _ => unreachable!(),
        };

        // Scan for cube rocks and insert piled-up round rocks behind each.
        for (start, rounds) in cubes {
            let range = match direction {
                Direction::N => start..start + rounds,
                Direction::S => todo!(),
                _ => unreachable!(),
            };
            for row in range {
                self.inner[row][column] = Cell::Round;
            }
        }
    }

    #[allow(unused_variables)]
    fn tilt_row(&mut self, row: usize, direction: Direction) {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, PartialEq)]
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

    fn dimensions(&self) -> Dimensions {
        Dimensions {
            r: self.inner.len(),
            c: self.inner[0].len(),
        }
    }
}

struct Dimensions {
    r: usize,
    c: usize,
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
