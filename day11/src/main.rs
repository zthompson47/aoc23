use std::collections::HashSet;

fn main() {
    println!("Part 1: {}", part1());
}

fn part1() -> usize {
    // Store universe.
    let lines = include_str!("input.txt").lines();
    //let lines = include_str!("test.txt").lines();
    let mut universe = lines.fold(Vec::new(), |mut acc, row| {
        let row = row.chars().map(|loc| loc == '#').collect::<Vec<_>>();
        acc.push(row);
        acc
    });

    // Find empty space.
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

    // Expand empty space.
    let mut empty_rows = empty_rows.iter().collect::<Vec<_>>();
    empty_rows.sort_by(|a, b| b.cmp(a));
    for i in empty_rows {
        universe.insert(*i, vec![false; universe[0].len()]);
    }
    let mut empty_columns = empty_columns.iter().collect::<Vec<_>>();
    empty_columns.sort_by(|a, b| b.cmp(a));
    for j in empty_columns.iter() {
        for row in universe.iter_mut() {
            row.insert(**j, false);
        }
    }

    // Find galaxies.
    let galaxies = universe
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, row)| {
            for (j, galaxy) in row.iter().enumerate() {
                if *galaxy {
                    acc.push((i, j));
                }
            }
            acc
        });

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_solution() {
        assert_eq!(9647174, part1());
    }
}
