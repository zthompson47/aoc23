use std::panic;

fn main() {
    let results = solve();
    println!("Part 1: {}", results.0);
    println!("Part 2: {}", results.1);
}

fn solve() -> (usize, usize) {
    let grid = include_str!("input.txt")
        .lines()
        .fold(Vec::new(), |mut acc, x| {
            acc.push(x.chars().map(Pipe::from).collect::<Vec<_>>());
            acc
        });

    let mut start = None;
    'out: for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if let Pipe::Start = grid[i][j] {
                start = Some(Position {
                    coords: (i, j),
                    from: None,
                    steps: 0,
                });
                break 'out;
            }
        }
    }
    let start = start.unwrap();

    let (mut path1, mut path2) = {
        let surrounding = Surrounding::from((&grid, start));
        let routes = surrounding.routes(&grid);
        assert_eq!(2, routes.len());
        (routes[0], routes[1])
    };

    let mut solution = vec![vec![Pipe::Ground; grid[0].len()]; grid.len()];
    solution[start.coords.0][start.coords.1] = {
        use Direction::*;
        match (direction(&start, &path1), direction(&start, &path2)) {
            (North, South) | (South, North) => Pipe::Vertical,
            (East, West) | (West, East) => Pipe::Horizontal,
            (North, East) | (East, North) => Pipe::NorthEast,
            (North, West) | (West, North) => Pipe::NorthWest,
            (South, East) | (East, South) => Pipe::SouthEast,
            (South, West) | (West, South) => Pipe::SouthWest,
            _ => panic!(),
        }
    };

    solution[path1.coords.0][path1.coords.1] = path1.pipe(&grid);
    solution[path2.coords.0][path2.coords.1] = path2.pipe(&grid);
    while path1.coords != path2.coords {
        path1 = path1.advance(&grid);
        path2 = path2.advance(&grid);
        solution[path1.coords.0][path1.coords.1] = path1.pipe(&grid);
        solution[path2.coords.0][path2.coords.1] = path2.pipe(&grid);
    }
    print_solution(fill_enclosed(solution.as_mut_slice()));

    let enclosed = solution
        .iter()
        .flatten()
        .filter(|x| **x == Pipe::Filled)
        .collect::<Vec<_>>()
        .len();

    (path1.steps, enclosed)
}

fn print_solution(solution: &[Vec<Pipe>]) {
    for row in solution {
        let row = row.iter().map(|x| x.as_char()).collect::<String>();
        println!("{row}");
    }
}

fn fill_enclosed(solution: &mut [Vec<Pipe>]) -> &[Vec<Pipe>] {
    use FillState::*;
    use Pipe::*;
    for row in (*solution).iter_mut() {
        let mut state = FillState::Outside;
        for pipe in row.iter_mut() {
            match pipe {
                Ground => match state {
                    Inside => *pipe = Filled,
                    Outside => (),
                    _ => panic!(),
                },

                NorthWest => match state {
                    Border(Direction::North) => state = Outside,
                    Border(Direction::South) => state = Inside,
                    _ => panic!(),
                },
                NorthEast => match state {
                    Outside => state = Border(Direction::North),
                    Inside => state = Border(Direction::South),
                    Border(_) => panic!(),
                },
                SouthWest => match state {
                    Border(Direction::North) => state = Inside,
                    Border(Direction::South) => state = Outside,
                    _ => panic!(),
                },
                SouthEast => match state {
                    Outside => state = Border(Direction::South),
                    Inside => state = Border(Direction::North),
                    Border(_) => panic!(),
                },

                Horizontal => match state {
                    Outside | Inside => panic!(),
                    _ => {}
                },
                Vertical => match state {
                    Outside => state = Inside,
                    Inside => state = Outside,
                    _ => panic!(),
                },

                Start => panic!(),
                Filled => panic!(),
            }
        }
    }

    solution
}

#[derive(Debug)]
enum FillState {
    Outside,
    Inside,
    Border(Direction),
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn direction(from: &Position, to: &Position) -> Direction {
    if from.coords.0 > to.coords.0 {
        Direction::North
    } else if from.coords.0 < to.coords.0 {
        Direction::South
    } else if from.coords.1 > to.coords.1 {
        Direction::West
    } else if from.coords.1 < to.coords.1 {
        Direction::East
    } else {
        panic!()
    }
}

type Grid = Vec<Vec<Pipe>>;

#[allow(unused)]
#[derive(Debug)]
struct Surrounding {
    north: Option<Position>,
    east: Option<Position>,
    south: Option<Position>,
    west: Option<Position>,
}

impl Surrounding {
    fn routes(&self, grid: &Grid) -> Vec<Position> {
        let mut results = Vec::new();

        if let Some(pos) = self.north {
            if pos.pipe(grid).open_south() {
                results.push(pos);
            }
        }

        if let Some(pos) = self.east {
            if pos.pipe(grid).open_west() {
                results.push(pos);
            }
        }

        if let Some(pos) = self.south {
            if pos.pipe(grid).open_north() {
                results.push(pos);
            }
        }

        if let Some(pos) = self.west {
            if pos.pipe(grid).open_east() {
                results.push(pos);
            }
        }

        results
    }
}

impl From<(&Grid, Position)> for Surrounding {
    fn from(value: (&Vec<Vec<Pipe>>, Position)) -> Self {
        let dimensions = (value.0.len(), value.0[0].len());
        let from_row = value.1.coords.0;
        let from_column = value.1.coords.1;

        let north = if from_row > 0 {
            Some(Position {
                coords: (from_row - 1, value.1.coords.1),
                from: Some(value.1.coords),
                steps: value.1.steps + 1,
            })
        } else {
            None
        };

        let south = if from_row < dimensions.0 - 1 {
            Some(Position {
                coords: (from_row + 1, value.1.coords.1),
                from: Some(value.1.coords),
                steps: value.1.steps + 1,
            })
        } else {
            None
        };

        let west = if from_column > 0 {
            Some(Position {
                coords: (from_row, from_column - 1),
                from: Some(value.1.coords),
                steps: value.1.steps + 1,
            })
        } else {
            None
        };

        let east = if from_column < dimensions.1 - 1 {
            Some(Position {
                coords: (from_row, from_column + 1),
                from: Some(value.1.coords),
                steps: value.1.steps + 1,
            })
        } else {
            None
        };

        Surrounding {
            north,
            east,
            south,
            west,
        }
    }
}

#[allow(unused)]
#[derive(Copy, Clone, Debug, PartialEq)]
struct Position {
    coords: (usize, usize),
    from: Option<(usize, usize)>,
    steps: usize,
}

impl Position {
    fn pipe(&self, grid: &Grid) -> Pipe {
        grid[self.coords.0][self.coords.1]
    }

    fn advance(&self, grid: &Grid) -> Self {
        let surrounding = Surrounding::from((grid, *self));
        let mut routes = Vec::new();

        if let Some(pos) = surrounding.north {
            if self.pipe(grid).open_north() && pos.pipe(grid).open_south() {
                routes.push(pos);
            }
        }

        if let Some(pos) = surrounding.south {
            if self.pipe(grid).open_south() && pos.pipe(grid).open_north() {
                routes.push(pos);
            }
        }

        if let Some(pos) = surrounding.east {
            if self.pipe(grid).open_east() && pos.pipe(grid).open_west() {
                routes.push(pos);
            }
        }

        if let Some(pos) = surrounding.west {
            if self.pipe(grid).open_west() && pos.pipe(grid).open_east() {
                routes.push(pos);
            }
        }

        for position in routes {
            if position.coords != self.from.unwrap() {
                return position;
            }
        }

        panic!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Pipe {
    Ground,
    NorthWest,
    NorthEast,
    Horizontal,
    SouthWest,
    SouthEast,
    Vertical,
    Start,
    Filled,
}

impl Pipe {
    fn open_south(&self) -> bool {
        [Pipe::Vertical, Pipe::SouthWest, Pipe::SouthEast].contains(self)
    }

    fn open_north(&self) -> bool {
        [Pipe::Vertical, Pipe::NorthWest, Pipe::NorthEast].contains(self)
    }

    fn open_west(&self) -> bool {
        [Pipe::Horizontal, Pipe::NorthWest, Pipe::SouthWest].contains(self)
    }

    fn open_east(&self) -> bool {
        [Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast].contains(self)
    }

    fn as_char(&self) -> char {
        match self {
            Pipe::Vertical => '┃',
            Pipe::Horizontal => '━',
            Pipe::NorthEast => '┗',
            Pipe::NorthWest => '┛',
            Pipe::SouthWest => '┓',
            Pipe::SouthEast => '┏',
            Pipe::Ground => ' ',
            Pipe::Start => 'S',
            Pipe::Filled => '*',
        }
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::Vertical,
            '-' => Pipe::Horizontal,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1() {
        assert_eq!((6768, 351), solve());
    }
}
