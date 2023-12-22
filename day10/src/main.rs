fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
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

    let (mut path1, mut path2) = if let Some(start) = start {
        let surrounding = Surrounding::from((&grid, start));
        let routes = surrounding.routes(&grid);

        assert_eq!(2, routes.len());

        (routes[0], routes[1])
    } else {
        panic!()
    };

    dbg!(start.unwrap().coords);
    dbg!(path1.coords);
    let mut step = 0;
    while path1.coords != path2.coords {
        path1 = path1.advance(&grid);
        path2 = path2.advance(&grid);
        dbg!(path1.coords);
        //if step == 7 {
        //    break;
        //}
        step += 1;
    }

    path1.steps
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

        dbg!(self.coords);
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
