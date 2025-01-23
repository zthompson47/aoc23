use aoc23::{Direction, Grid, Position};

fn main() {
    let grid: Grid<Cell> = Grid::from(include_str!("input.txt"));

    println!("Part 1: {}", run(Position::new(0, 0), Direction::E, &grid));

    let mut max_score = 0;
    for r in 0..grid.dim().r {
        max_score = max_score.max(run(Position::new(r, 0), Direction::E, &grid));
        max_score = max_score.max(run(Position::new(r, grid.dim().c - 1), Direction::W, &grid));
    }
    for c in 0..grid.dim().c {
        max_score = max_score.max(run(Position::new(0, c), Direction::S, &grid));
        max_score = max_score.max(run(Position::new(grid.dim().r - 1, c), Direction::N, &grid));
    }
    println!("Part 2: {}", max_score);
}

fn run(start: Position, direction: Direction, grid: &Grid<Cell>) -> usize {
    let mut visited: Visited = vec![vec![Vec::new(); grid.dim().r]; grid.dim().c];
    let mut beams: Vec<Beam> = vec![Beam {
        position: start,
        direction,
        is_active: true,
    }];
    visited[start.r][start.c].push(direction);

    while !beams
        .iter()
        .filter(|x| x.is_active)
        .collect::<Vec<_>>()
        .is_empty()
    {
        let mut new_beams = Vec::new();
        for beam in beams.iter_mut().filter(|x| x.is_active) {
            if let Some(generated) = beam.step(grid, &mut visited) {
                new_beams.push(generated);
            }
        }
        for new_beam in new_beams {
            beams.push(new_beam);
        }
    }
    //display_visited_grid(grid, &visited);
    visited.iter().flatten().filter(|x| !x.is_empty()).count()
}

type Visited = Vec<Vec<Vec<Direction>>>;

#[derive(Debug)]
struct Beam {
    position: Position,
    direction: Direction,
    is_active: bool,
}

impl Beam {
    fn step(&mut self, grid: &Grid<Cell>, visited: &mut Visited) -> Option<Beam> {
        assert!(self.is_active);
        let mut extra_beam: Option<Beam> = None;

        match grid.cell(self.position) {
            Cell::MirrorBack => match self.direction {
                Direction::N => self.direction = Direction::W,
                Direction::E => self.direction = Direction::S,
                Direction::S => self.direction = Direction::E,
                Direction::W => self.direction = Direction::N,
            },
            Cell::MirrorForward => match self.direction {
                Direction::N => self.direction = Direction::E,
                Direction::E => self.direction = Direction::N,
                Direction::S => self.direction = Direction::W,
                Direction::W => self.direction = Direction::S,
            },
            Cell::SplitterVertical => match self.direction {
                Direction::N => {}
                Direction::S => {}
                Direction::E | Direction::W => {
                    self.direction = Direction::N;
                    if let Some(position) = self.position.step(Direction::S, grid) {
                        extra_beam = Some(Beam {
                            position,
                            direction: Direction::S,
                            is_active: true,
                        });
                    }
                }
            },
            Cell::SplitterHorizontal => match self.direction {
                Direction::E => {}
                Direction::W => {}
                Direction::N | Direction::S => {
                    self.direction = Direction::E;
                    if let Some(position) = self.position.step(Direction::W, grid) {
                        extra_beam = Some(Beam {
                            position,
                            direction: Direction::W,
                            is_active: true,
                        });
                    }
                }
            },
            Cell::Empty => {}
        }

        // Try to move a step.
        if let Some(next_position) = self.position.step(self.direction, grid) {
            self.position = next_position;
        } else {
            self.is_active = false;
        }

        // Check for loops.
        if visited[self.position.r][self.position.c].contains(&self.direction) {
            self.is_active = false;
        } else {
            visited[self.position.r][self.position.c].push(self.direction);
        }

        // Check for loops in extra beam.
        if let Some(extra_beam) = extra_beam {
            if visited[extra_beam.position.r][extra_beam.position.c].contains(&extra_beam.direction)
            {
                None
            } else {
                visited[extra_beam.position.r][extra_beam.position.c].push(extra_beam.direction);
                Some(extra_beam)
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[allow(unused)]
fn display_visited_grid(grid: &Grid<Cell>, visited: &Visited) {
    use colored::Colorize;
    let mut count = 0;
    for r in 0..visited.len() {
        for c in 0..visited[0].len() {
            if visited[r][c].is_empty() {
                print!("{}", char::from(grid.cell(Position { r, c })));
            } else if visited[r][c].len() > 2 {
                count += 1;
                print!(
                    "{}",
                    char::from(grid.cell(Position { r, c }))
                        .to_string()
                        .as_str()
                        .purple()
                        .reversed()
                );
            } else if visited[r][c].len() > 1 {
                count += 1;
                print!(
                    "{}",
                    char::from(&grid.cell(Position { r, c }))
                        .to_string()
                        .as_str()
                        .green()
                        .reversed()
                );
            } else if !visited[r][c].is_empty() {
                count += 1;
                let cell = match visited[r][c][0] {
                    Direction::N => '^',
                    Direction::S => 'v',
                    Direction::E => '>',
                    Direction::W => '<',
                };
                print!(
                    "{}",
                    //cell
                    char::from(grid.cell(Position { r, c }))
                        .to_string()
                        .as_str()
                        .red()
                        .reversed()
                );
            }
        }
        println!();
    }
    println!("=========================={count}====================================");
}
