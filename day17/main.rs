use aoc23::{Alignment, Direction, Grid, Position};

fn main() {
    let mut grid = Grid::<Number>::from(include_str!("test.txt"));
    println!("{grid}");
    println!("{:?}", grid.dim());

    let start = Position::new(0, 0);
    let east_phase = Phase {
        position: start,
        direction: Direction::E,
    };
    let south_phase = Phase {
        position: start,
        direction: Direction::S,
    };

    let east = min_path(east_phase, &mut grid, Vec::new(), 110);
    let south = min_path(south_phase, &mut grid, Vec::new(), 110);
    let part1 = east.min(south).unwrap();
    println!("--e:{east:?}--s:{south:?}--");

    println!("Part 1: {part1}");
}

fn min_path(
    from: Phase,
    grid: &mut Grid<Number>,
    mut current_path: Vec<Phase>,
    max: u32,
) -> Option<u32> {
    //    println!("min_path() from:{from:?}");

    // Stop searching at some maximum value.
    if current_path
        .iter()
        .map(|x| grid.cell(x.position).inner)
        .sum::<u32>()
        > max
    {
        return None;
    }

    // No loops.
    if current_path.contains(&from) {
        //       println!("Found loop: {from:?}");
        return None;
    } else {
        current_path.push(from);
    }

    // End case - found solution cell.
    if from.position == grid.bottom_right() {
        println!(
            "END CASE - heat:{}",
            current_path
                .iter()
                .map(|x| grid.cell(x.position).inner)
                .sum::<u32>()
        );

        // Print colorized map of path for debugging.
        use colored::Colorize;
        grid.print(|position: Position, c: char| {
            if current_path.iter().any(|p| p.position == position) {
                c.to_string().red().to_string()
            } else {
                c.to_string()
            }
        });

        return Some(0);
    }

    // Check for cached results.
    match from.direction.alignment() {
        Alignment::Vertical => {
            if let Some(result) = grid.cell(from.position).visited_vertical {
                //                println!("    LOOP VERTICAL");
                //                return Some(result);
            }
        }
        Alignment::Horizontal => {
            if let Some(result) = grid.cell(from.position).visited_horizontal {
                //                println!("    LOOP HORIZONTAL");
                //                return Some(result);
            }
        }
    }

    // Recursion to find all paths.
    let mut result: Option<u32> = None;

    for direction in from.direction.orthogonal() {
        let mut path_heat = 0;

        for position in from.position.steps(3, direction, grid) {
            path_heat += grid.cell(position).inner;
            /*
            println!(
                "  search step {position:?} heat:{path_heat} from:{:?}",
                from.position
            );
            */
            if let Some(mut new_min) = min_path(
                Phase {
                    position,
                    direction,
                },
                grid,
                current_path.clone(),
                max,
            ) {
                //                println!("GOT ONE {new_min:?}");
                new_min += path_heat;
                result = if let Some(old_result) = result {
                    Some(old_result.min(new_min))
                } else {
                    Some(new_min)
                };
            }
        }
    }

    if let Some(result) = result {
        // Cache result.
        match from.direction.alignment() {
            Alignment::Vertical => {
                grid.cell_mut(from.position).visited_vertical = Some(result);
            }
            Alignment::Horizontal => {
                grid.cell_mut(from.position).visited_horizontal = Some(result);
            }
        }

        Some(result)
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Phase {
    position: Position,
    direction: Direction,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Number {
    inner: u32,
    visited_horizontal: Option<u32>,
    visited_vertical: Option<u32>,
}

impl From<char> for Number {
    fn from(value: char) -> Self {
        Number {
            inner: value.to_digit(10).unwrap(),
            visited_horizontal: None,
            visited_vertical: None,
        }
    }
}

impl From<&Number> for char {
    fn from(value: &Number) -> Self {
        value.inner.to_string().chars().collect::<Vec<_>>()[0]
    }
}
