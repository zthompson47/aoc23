use std::fmt::Display;

fn main() {
    let mut mirrors = include_str!("input.txt")
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
        });

    let mut score = 0;
    for mirror in mirrors.iter_mut() {
        println!(
            "rows: {}, columns: {}",
            mirror.rows.len(),
            mirror.columns.len()
        );
        mirror.calculate_symmetry();
        score += mirror.score();
        println!("{mirror}");
    }

    println!("Part 1: {}", score);
}

#[derive(Default, Debug)]
struct Mirror {
    rows: Vec<Vec<Cell>>,
    columns: Vec<Vec<Cell>>,
    symmetry: Symmetry,
}

impl Mirror {
    /// Check every possible symmetry line in rows and columns.
    /// Assume there is a unique line of symmetry for each mirror.
    fn calculate_symmetry(&mut self) {
        'rows: for i in 1..self.rows.len() {
            for j in 1..i.min(self.rows.len() - i) + 1 {
                if self.rows[i - j] != self.rows[i - 1 + j] {
                    continue 'rows;
                }
            }
            assert_eq!(self.symmetry, Symmetry::Unknown);
            self.symmetry = Symmetry::Row(i);
            return;
        }
        'columns: for i in 1..self.columns.len() {
            for j in 1..i.min(self.columns.len() - i) + 1 {
                if self.columns[i - j] != self.columns[i - 1 + j] {
                    continue 'columns;
                }
            }
            assert_eq!(self.symmetry, Symmetry::Unknown);
            self.symmetry = Symmetry::Column(i);
            return;
        }
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
