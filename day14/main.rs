use std::collections::HashMap;

fn main() {
    let mut grid: Grid<Cell> = Grid::from(include_str!("input.txt"));
    grid.tilt(Direction::N);
    println!("Part 1: {}", grid.score());

    // Spin the grid until we find it cycled back to a prior state, then calculate
    // final state from modulo.
    let mut grid: Grid<Cell> = Grid::from(include_str!("test.txt"));
    let mut cycle_cache: Vec<Grid<Cell>> = Vec::new();
    let mut repeat = None;
    let cycles = 1_000_000_000;
    for i in 0..cycles {
        grid.tilt(Direction::N);
        grid.tilt(Direction::W);
        grid.tilt(Direction::S);
        grid.tilt(Direction::E);
        if cycle_cache.contains(&grid) {
            repeat = Some(i);
            cycle_cache.push(grid.clone());
            break;
        }
        cycle_cache.push(grid.clone());
    }
    if let Some(repeat) = repeat {
        let first_repeat = cycle_cache
            .iter()
            .position(|x| x == cycle_cache.last().unwrap())
            .unwrap();
        println!("first_repeat:{first_repeat}, len cache: {}", cycle_cache.len());
        for grid in &cycle_cache {
            println!("-->> {}", grid.score());
        }
        let index = (cycles - first_repeat - 1) % (cycle_cache.len() - first_repeat - 1);
        println!("index -->> {index}");
        grid = cycle_cache[first_repeat..cycle_cache.len() - 1][index].clone();

        //let index = (cycles - first_repeat) % (cycle_cache.len() - first_repeat - 2) + first_repeat;
        //grid = cycle_cache[first_repeat..][index].clone();
        //let index = (cycles - first_repeat) % (cycle_cache.len() - 1 - first_repeat);
        //grid = cycle_cache[first_repeat..cycle_cache.len() - 2][index].clone();
    }
    println!("Part 2: {}", grid.score());
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
                Direction::S => row,
                _ => unreachable!(),
            };
            match self.inner[row][column] {
                Cell::Round => {
                    rounds += 1;
                    self.inner[row][column] = Cell::Empty;
                }
                Cell::Cube => {
                    if rounds > 0 {
                        cubes.insert(
                            (row as isize
                                + match direction {
                                    Direction::N => 1,
                                    Direction::S => -1,
                                    _ => unreachable!(),
                                }) as usize,
                            rounds,
                        );
                        rounds = 0;
                    }
                }
                Cell::Empty => {}
            }
        }
        match direction {
            Direction::N => cubes.insert(0, rounds),
            Direction::S => cubes.insert(self.dimensions().r - 1, rounds),
            _ => unreachable!(),
        };

        // Scan for cube rocks and insert piled-up round rocks behind each.
        for (start, rounds) in cubes {
            let range = match direction {
                Direction::N => start..start + rounds,
                Direction::S => (start as isize - rounds as isize + 1) as usize..start + 1,
                _ => unreachable!(),
            };
            for row in range {
                self.inner[row][column] = Cell::Round;
            }
        }
    }

    fn tilt_row(&mut self, row: usize, direction: Direction) {
        // Scan for round rocks and store pile-up points at cube rocks.
        let mut cubes: HashMap<usize, usize> = HashMap::new();
        let mut rounds = 0;
        for mut column in 0..self.dimensions().c {
            column = match direction {
                Direction::W => self.dimensions().c - column - 1,
                Direction::E => column,
                _ => unreachable!(),
            };
            match self.inner[row][column] {
                Cell::Round => {
                    rounds += 1;
                    self.inner[row][column] = Cell::Empty;
                }
                Cell::Cube => {
                    if rounds > 0 {
                        cubes.insert(
                            (column as isize
                                + match direction {
                                    Direction::W => 1,
                                    Direction::E => -1,
                                    _ => unreachable!(),
                                }) as usize,
                            rounds,
                        );
                        rounds = 0;
                    }
                }
                Cell::Empty => {}
            }
        }
        match direction {
            Direction::W => cubes.insert(0, rounds),
            Direction::E => cubes.insert(self.dimensions().c - 1, rounds),
            _ => unreachable!(),
        };

        // Scan for cube rocks and insert piled-up round rocks behind each.
        for (start, rounds) in cubes {
            let range = match direction {
                Direction::W => start..start + rounds,
                Direction::E => (start as isize - rounds as isize + 1) as usize..start + 1,
                _ => unreachable!(),
            };
            for column in range {
                self.inner[row][column] = Cell::Round;
            }
        }
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Clone, PartialEq)]
struct Grid<T>
where
    T: Clone,
{
    inner: Vec<Vec<T>>,
}

impl<T> Grid<T>
where
    T: Clone,
{
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
    T: Clone,
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
    T: Clone,
{
    fn from(value: &str) -> Self {
        value.lines().fold(Grid::new(), |mut grid, row| {
            grid.inner.push(row.chars().map(T::from).collect());
            grid
        })
    }
}
