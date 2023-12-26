use std::collections::HashSet;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

type Universe = Vec<Vec<bool>>;

fn create_universe() -> Universe {
    let lines = include_str!("input.txt").lines();
    lines.fold(Vec::new(), |mut acc, row| {
        let row = row.chars().map(|loc| loc == '#').collect::<Vec<_>>();
        acc.push(row);
        acc
    })
}

fn empty_space(universe: &Universe) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = (0..universe.len()).collect::<HashSet<_>>();
    let mut empty_columns = (0..universe[0].len()).collect::<HashSet<_>>();

    for (i, row) in universe.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if universe[i][j] {
                empty_rows.remove(&i);
                empty_columns.remove(&j);
            }
        }
    }

    let mut empty_rows = empty_rows.iter().copied().collect::<Vec<_>>();
    empty_rows.sort_by(|a, b| b.cmp(a));

    let mut empty_columns = empty_columns.iter().copied().collect::<Vec<_>>();
    empty_columns.sort_by(|a, b| b.cmp(a));

    (empty_rows, empty_columns)
}

fn galaxies(universe: &Universe) -> Vec<(usize, usize)> {
    universe
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, row)| {
            for (j, galaxy) in row.iter().enumerate() {
                if *galaxy {
                    acc.push((i, j));
                }
            }
            acc
        })
}

fn distance(empty_space: (&[usize], &[usize]), from: (usize, usize), to: (usize, usize)) -> usize {
    let row_factor = empty_space
        .0
        .iter()
        .filter(|x| {
            let upper = from.0.max(to.0);
            let lower = from.0.min(to.0);
            x < &&upper && x > &&lower
        })
        .collect::<Vec<&usize>>()
        .len();

    let column_factor = empty_space
        .1
        .iter()
        .filter(|x| {
            let upper = from.1.max(to.1);
            let lower = from.1.min(to.1);
            x < &&upper && x > &&lower
        })
        .collect::<Vec<&usize>>()
        .len();

    from.0.abs_diff(to.0)
        + row_factor * (1000000 - 1)
        + from.1.abs_diff(to.1)
        + column_factor * (1000000 - 1)
}

fn part1() -> usize {
    let mut universe = create_universe();
    let (empty_rows, empty_columns) = empty_space(&universe);

    // Expand empty space.
    for i in empty_rows {
        universe.insert(i, vec![false; universe[0].len()]);
    }
    for j in empty_columns.iter() {
        for row in universe.iter_mut() {
            row.insert(*j, false);
        }
    }

    let galaxies = galaxies(&universe);

    // Calculate sum of shortest paths between all pairs of galaxies.
    let mut result = 0;
    let mut galaxies = galaxies.as_slice();
    while let Some((first, rest)) = galaxies.split_first() {
        result += rest
            .iter()
            .map(|(row, column)| {
                (*row as i32 - first.0 as i32).abs() + (*column as i32 - first.1 as i32).abs()
            })
            .sum::<i32>() as usize;
        galaxies = rest;
    }

    result
}

fn part2() -> usize {
    let universe = create_universe();
    let (empty_rows, empty_columns) = empty_space(&universe);
    let _galaxies = galaxies(&universe);
    let mut galaxies = _galaxies.as_slice();
    let mut result = 0;

    while let Some(((row0, column0), rest)) = galaxies.split_first() {
        result += rest
            .iter()
            .map(|(row, column)| {
                distance(
                    (&empty_rows, &empty_columns),
                    (*row, *column),
                    (*row0, *column0),
                )
            })
            .sum::<usize>();
        galaxies = rest;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_solution() {
        assert_eq!(9647174, part1());
    }

    #[test]
    fn pt2_solution() {
        assert_eq!(377318892554, part2());
    }
}
