#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    pub inner: Vec<Vec<T>>,
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Self::default()
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

    pub fn swap_first(&mut self, target: T, replacement: T) -> Option<Position>
    where
        T: PartialEq,
    {
        for (r_i, row) in self.inner.iter_mut().enumerate() {
            for (c_i, square) in row.iter_mut().enumerate() {
                if *square == target {
                    *square = replacement;
                    return Some(Position::new(r_i, c_i));
                }
            }
        }
        None
    }

    pub fn take_first(&mut self, target: T) -> Option<Position>
    where
        T: PartialEq + Default,
    {
        for (r_i, row) in self.inner.iter_mut().enumerate() {
            for (c_i, square) in row.iter_mut().enumerate() {
                if *square == target {
                    *square = T::default();
                    return Some(Position::new(r_i, c_i));
                }
            }
        }
        None
    }

    pub fn square(&self, size: usize) -> Self
    where
        T: Copy,
    {
        let mut horizontal = self.clone();
        for _ in 0..size - 1 {
            horizontal.extend(Alignment::Horizontal, self.clone());
        }
        let mut squared = horizontal.clone();
        for _ in 0..size - 1 {
            squared.extend(Alignment::Vertical, horizontal.clone());
        }
        squared
    }

    pub fn extend(&mut self, alignment: Alignment, grid: Self)
    where
        T: Copy,
    {
        //assert_eq!(self.dim(), grid.dim());
        match alignment {
            Alignment::Horizontal => {
                self.inner
                    .iter_mut()
                    .zip(grid.inner.iter())
                    .for_each(|(this, other)| {
                        this.extend(other);
                    })
            }

            Alignment::Vertical => {
                self.inner.extend(grid.inner);
            }
        }
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

    pub fn adjacent<T>(&self, grid: &Grid<T>) -> Vec<Self> {
        let mut result = vec![];
        for direction in Direction::all() {
            if let Some(step) = self.step(direction, grid) {
                result.push(step);
            }
        }
        result
    }

    pub fn adjacent_wrapping<T>(&self, grid: &Grid<T>) -> [(Self, Option<Direction>); 4] {
        Direction::all()
            .iter()
            .fold(vec![], |mut result, direction| {
                result.push(self.step_wrapping(*direction, grid));
                result
            })
            .try_into()
            .unwrap()
    }

    pub fn adjacent_if_wrapping<T>(
        &self,
        grid: &Grid<T>,
        predicate: impl Fn(&T) -> bool,
    ) -> Vec<(Self, Option<Direction>)> {
        Direction::all()
            .iter()
            .fold(vec![], |mut result, direction| {
                let step = self.step_wrapping(*direction, grid);
                if predicate(grid.cell(step.0)) {
                    result.push(self.step_wrapping(*direction, grid));
                }
                result
            })
    }

    pub fn adjacent_if_wrapping2<T>(
        &self,
        grid: &Grid<T>,
        result: &mut Vec<(Self, Option<Direction>)>,
        predicate: impl Fn(&T) -> bool,
    ) {
        Direction::all().iter().for_each(|direction| {
            let step = self.step_wrapping(*direction, grid);
            if predicate(grid.cell(step.0)) {
                result.push(self.step_wrapping(*direction, grid));
            }
        })
    }

    pub fn adjacent_if<T>(&self, grid: &Grid<T>, predicate: impl Fn(&T) -> bool) -> Vec<Self> {
        let mut result = vec![];

        for direction in Direction::all() {
            if let Some(step) = self.step(direction, grid) {
                if predicate(grid.cell(step)) {
                    result.push(step);
                }
            }
        }
        result
    }

    pub fn adjacent_if_direction<T>(
        &self,
        grid: &Grid<T>,
        predicate: impl Fn(&T, Direction) -> bool,
    ) -> Vec<(Self, Direction)> {
        let mut result = vec![];

        for direction in Direction::all() {
            if let Some(step) = self.step(direction, grid) {
                if predicate(grid.cell(step), direction) {
                    result.push((step, direction));
                }
            }
        }
        result
    }

    pub fn steps<T>(&self, count: usize, direction: Direction, grid: &Grid<T>) -> Vec<Self> {
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

    pub fn step_fallible(&self, direction: Direction) -> Self {
        match direction {
            Direction::N => Position {
                r: self.r - 1,
                c: self.c,
            },
            Direction::E => Position {
                r: self.r,
                c: self.c + 1,
            },
            Direction::S => Position {
                r: self.r + 1,
                c: self.c,
            },
            Direction::W => Position {
                r: self.r,
                c: self.c - 1,
            },
        }
    }

    pub fn step<T>(&self, direction: Direction, grid: &Grid<T>) -> Option<Self> {
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

    pub fn steps_wrapping<T>(
        &self,
        count: usize,
        direction: Direction,
        grid: &Grid<T>,
    ) -> Vec<(Self, Direction, usize)> {
        let mut result = Vec::new();
        let mut start = *self;
        let mut wraps = 0;
        for _ in 0..count {
            let (position, wrap_direction) = start.step_wrapping(direction, grid);
            if wrap_direction.is_some() {
                wraps += 1;
            }
            result.push((position, direction, wraps));
            start = position;
        }
        result
    }

    pub fn step_wrapping<T>(
        &self,
        direction: Direction,
        grid: &Grid<T>,
    ) -> (Self, Option<Direction>) {
        match direction {
            Direction::N => {
                if self.r > 0 {
                    (
                        Position {
                            r: self.r - 1,
                            c: self.c,
                        },
                        None,
                    )
                } else {
                    (
                        Position {
                            r: grid.dim().r - 1,
                            c: self.c,
                        },
                        Some(direction),
                    )
                }
            }
            Direction::E => {
                if self.c < grid.dim().c - 1 {
                    (
                        Position {
                            r: self.r,
                            c: self.c + 1,
                        },
                        None,
                    )
                } else {
                    (Position { r: self.r, c: 0 }, Some(direction))
                }
            }
            Direction::S => {
                if self.r < grid.dim().r - 1 {
                    (
                        Position {
                            r: self.r + 1,
                            c: self.c,
                        },
                        None,
                    )
                } else {
                    (Position { r: 0, c: self.c }, Some(direction))
                }
            }
            Direction::W => {
                if self.c > 0 {
                    (
                        Position {
                            r: self.r,
                            c: self.c - 1,
                        },
                        None,
                    )
                } else {
                    (
                        Position {
                            r: self.r,
                            c: grid.dim().c - 1,
                        },
                        Some(direction),
                    )
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dimensions {
    pub r: usize,
    pub c: usize,
}

impl Dimensions {
    pub fn new(r: usize, c: usize) -> Self {
        Dimensions { r, c }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl std::fmt::Display for Dimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:2}, {:2})", self.r, self.c)
    }
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

    fn assert_vec_eq<T>(v1: Vec<T>, v2: Vec<T>)
    where
        T: PartialEq,
    {
        assert_eq!(v1.len(), v2.len());
        for item in v1 {
            assert!(v2.contains(&item));
        }
    }

    // . . . . .
    // . # . # .
    // . . # . .
    // . # . . .
    // . # # # .
    fn new_grid() -> Grid<Cell> {
        let input = ".....\n.#.#.\n..#..\n.#...\n.###.\n";
        Grid::<Cell>::from(input)
    }

    #[test]
    fn grid_load_and_display() {
        let input = "..#.\n..#.\n....\n...#\n";
        let grid = Grid::<Cell>::from(input);
        assert_eq!(input.trim(), format!("{grid}"));
    }

    #[test]
    fn steps_wrapping() {
        let grid = new_grid();
        let position = Position::new(0, 4);
        assert_eq!(
            position.steps_wrapping(2, Direction::N, &grid),
            vec![
                (Position::new(4, 4), Direction::N, 1),
                (Position::new(3, 4), Direction::N, 1)
            ]
        );
        assert_eq!(
            position.steps_wrapping(6, Direction::N, &grid),
            vec![
                (Position::new(4, 4), Direction::N, 1),
                (Position::new(3, 4), Direction::N, 1),
                (Position::new(2, 4), Direction::N, 1),
                (Position::new(1, 4), Direction::N, 1),
                (Position::new(0, 4), Direction::N, 1),
                (Position::new(4, 4), Direction::N, 2),
            ]
        );
    }

    #[test]
    fn step_wrapping() {
        let grid = new_grid();
        let position = Position::new(0, 4);
        assert_eq!(
            (Position::new(4, 4), Some(Direction::N)),
            position.step_wrapping(Direction::N, &grid)
        );
        assert_eq!(
            (Position::new(0, 0), Some(Direction::E)),
            position.step_wrapping(Direction::E, &grid)
        );
        assert_eq!(
            (Position::new(1, 4), None),
            position.step_wrapping(Direction::S, &grid)
        );
        assert_eq!(
            (Position::new(0, 3), None),
            position.step_wrapping(Direction::W, &grid)
        );
        let position = Position::new(4, 0);
        assert_eq!(
            (Position::new(3, 0), None),
            position.step_wrapping(Direction::N, &grid)
        );
        assert_eq!(
            (Position::new(4, 1), None),
            position.step_wrapping(Direction::E, &grid)
        );
        assert_eq!(
            (Position::new(0, 0), Some(Direction::S)),
            position.step_wrapping(Direction::S, &grid)
        );
        assert_eq!(
            (Position::new(4, 4), Some(Direction::W)),
            position.step_wrapping(Direction::W, &grid)
        );
    }

    #[test]
    fn adjacent_wrapping() {
        let grid = new_grid();
        let position = Position::new(1, 4);

        assert_vec_eq(
            [
                (Position::new(0, 4), None),
                (Position::new(2, 4), None),
                (Position::new(1, 3), None),
                (Position::new(1, 0), Some(Direction::E)),
            ]
            .to_vec(),
            position.adjacent_wrapping(&grid).to_vec(),
        );

        assert_vec_eq(
            vec![
                (Position::new(0, 4), None),
                (Position::new(2, 4), None),
                (Position::new(1, 0), Some(Direction::E)),
            ],
            position
                .adjacent_if_wrapping(&grid, |c| *c == Cell::Empty)
                .to_vec(),
        );
    }

    #[test]
    fn adjacent_positions() {
        let grid = new_grid();
        let position = Position::new(1, 4);

        assert_vec_eq(
            [
                Position::new(0, 4),
                Position::new(2, 4),
                Position::new(1, 3),
            ]
            .into(),
            position.adjacent(&grid),
        );

        assert_vec_eq(
            [Position::new(0, 4), Position::new(2, 4)].into(),
            position.adjacent_if(&grid, |c| *c == Cell::Empty),
        );
    }

    #[test]
    fn extend_grid() {
        let mut grid = new_grid();
        let dim = grid.dim();
        grid.extend(Alignment::Horizontal, grid.clone());
        assert_eq!(grid.dim().c, dim.c * 2);
        grid.extend(Alignment::Vertical, grid.clone());
        assert_eq!(grid.dim().r, dim.r * 2);
    }
}
