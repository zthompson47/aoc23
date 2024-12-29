use std::fmt::Display;

fn main() {
    let mut mirrors = input();
    let mut score = 0;
    for mirror in mirrors.iter_mut() {
        mirror.calculate_symmetry(false);
        score += mirror.score();
    }

    println!("Part 1: {}", score);

    let mut score = 0;
    'mirrors: for mirror in mirrors.iter_mut() {
        for row in 0..mirror.rows.len() {
            for column in 0..mirror.columns.len() {
                // Repeat flip for redundant column/row storage.
                mirror.rows[row][column] = mirror.rows[row][column].flip();
                mirror.columns[column][row] = mirror.columns[column][row].flip();

                if mirror.calculate_symmetry(true) {
                    score += mirror.score();
                    continue 'mirrors;
                }

                // Unflip..
                mirror.rows[row][column] = mirror.rows[row][column].flip();
                mirror.columns[column][row] = mirror.columns[column][row].flip();
            }
        }
    }

    println!("Part 2: {}", score);
}

fn input() -> Vec<Mirror> {
    include_str!("input.txt")
        .lines()
        .fold(Vec::new(), |mut mirrors, line| {
            if mirrors.is_empty() || line.is_empty() {
                mirrors.push(Mirror::default());
            }
            if line.is_empty() {
                return mirrors;
            }
            if let Some(mirror) = mirrors.last_mut() {
                let row: Vec<Cell> = line.chars().map(Cell::from).collect();
                if mirror.columns.is_empty() {
                    mirror.columns = vec![Vec::new(); row.len()];
                }
                mirror.rows.push(row);
                if let Some(row) = mirror.rows.last() {
                    for (i, cell) in row.iter().enumerate() {
                        mirror.columns[i].push(cell.clone());
                    }
                }
            }
            mirrors
        })
}

#[derive(Default, Debug)]
struct Mirror {
    rows: Vec<Vec<Cell>>,
    columns: Vec<Vec<Cell>>,
    symmetry: Symmetry,
}

impl Mirror {
    /// Check every possible symmetry line in rows and columns.
    /// Assume there is a unique line of symmetry for each mirror,
    /// or there is no symmetry.
    fn calculate_symmetry(&mut self, replace: bool) -> bool {
        'rows: for i in 1..self.rows.len() {
            for j in 1..i.min(self.rows.len() - i) + 1 {
                if self.rows[i - j] != self.rows[i - 1 + j] {
                    continue 'rows;
                }
            }
            if replace && self.symmetry == Symmetry::Row(i) {
                continue 'rows;
            } else {
                self.symmetry = Symmetry::Row(i);
                return true;
            }
        }
        'columns: for i in 1..self.columns.len() {
            for j in 1..i.min(self.columns.len() - i) + 1 {
                if self.columns[i - j] != self.columns[i - 1 + j] {
                    continue 'columns;
                }
            }
            if replace && self.symmetry == Symmetry::Column(i) {
                continue 'columns;
            } else {
                self.symmetry = Symmetry::Column(i);
                return true;
            }
        }
        if !replace {
            self.symmetry = Symmetry::Unknown;
        }
        false
    }

    fn score(&self) -> usize {
        match self.symmetry {
            Symmetry::Unknown => unreachable!(),
            Symmetry::Row(x) => x * 100,
            Symmetry::Column(x) => x,
        }
    }
}

impl Display for Mirror {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.clone() {
            writeln!(f, "{}", row.iter().map(Cell::to_char).collect::<String>())?
        }
        writeln!(f, "Symmetry: {:?}", self.symmetry)
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Ash,
    Rocks,
}

impl Cell {
    fn to_char(&self) -> char {
        match self {
            Cell::Ash => '.',
            Cell::Rocks => '#',
        }
    }

    fn flip(&self) -> Self {
        match self {
            Cell::Ash => Cell::Rocks,
            Cell::Rocks => Cell::Ash,
        }
    }
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Ash,
            '#' => Cell::Rocks,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
enum Symmetry {
    #[default]
    Unknown,
    Row(usize),
    Column(usize),
}
