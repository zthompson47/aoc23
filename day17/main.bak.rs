use aoc23::{Alignment, Direction, Grid, Position};

fn main() {
    let mut grid = Grid::<Number>::from(include_str!("test.txt"));
    //println!("{grid}");
    //println!("{:?}", grid.dim());

    let start = Position::new(0, 0);
    let east_phase = Phase {
        position: start,
        direction: Direction::E,
    };
    let south_phase = Phase {
        position: start,
        direction: Direction::S,
    };

    //let east = min_path(east_phase, &mut grid, Vec::new(), 110);
    //let south = min_path(south_phase, &mut grid, Vec::new(), 110);
    //let part1 = east.min(south).unwrap();
    //println!("--e:{east:?}--s:{south:?}--");

    let east_start = Path {
        phases: vec![east_phase],
        positions: vec![],
        finished: false,
        max_heat: None,
    };
    let south_start = Path {
        phases: vec![south_phase],
        positions: vec![],
        finished: false,
        max_heat: None,
    };

    //let paths = vec![east_path, south_path];
    //let part1: u32 = min_path_wide(paths, &mut grid)[0].heat(&grid);

    //let mut east_solutions = Vec::new();
    //let mut south_solutions = Vec::new();
    //minimum_path(east_path, &mut grid, &mut east_solutions);
    //minimum_path(south_path, &mut grid, &mut south_solutions);
    //let sol1 = grid.cell(grid.top_left()).solved_vertical[0].heat(&grid);
    //let sol2 = grid.cell(grid.top_left()).solved_horizontal[0].heat(&grid);
    //let part1 = sol1.min(sol2);
    //let part1 = east_solutions[0]
    //    .heat(&grid)
    //    .min(south_solutions[0].heat(&grid));

    let east = find_min_path(east_start, &mut grid);
    let south = find_min_path(south_start, &mut grid);
    let part1 = east.min(south).unwrap();
    println!("Part 1: {part1}");
}

#[derive(Clone, Debug, PartialEq)]
struct Path {
    phases: Vec<Phase>,
    positions: Vec<Position>,
    finished: bool,
    max_heat: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Phase {
    position: Position,
    direction: Direction,
}

#[derive(Clone, Debug, PartialEq)]
struct Number {
    inner: u32,
    visited_horizontal: Option<u32>,
    visited_vertical: Option<u32>,
    solved_horizontal: Vec<Path>,
    solved_vertical: Vec<Path>,
}

fn find_min_path(start: Path, grid: &mut Grid<Number>) -> Option<u32> {
    let phase = start.phases.last().unwrap();
    //println!("find_min_path {phase:?}");

    /*
    use colored::Colorize;
    grid.print(|position: Position, c: char| {
        if grid.cell(position).visited_horizontal.is_some()
            || grid.cell(position).visited_vertical.is_some()
        {
            //c.to_string().green().reversed().to_string()
            "X".into()
        } else if start.positions.iter().any(|p| *p == position) {
            //c.to_string().red().to_string()
            ".".into()
        } else {
            c.to_string()
        }
    });
    println!();
    */

    match phase.direction.alignment() {
        Alignment::Horizontal => {
            if let Some(min_heat) = grid.cell(phase.position).visited_horizontal {
                println!("  found cached horizontal {min_heat}");
                return Some(min_heat);
            }
        }
        Alignment::Vertical => {
            if let Some(min_heat) = grid.cell(phase.position).visited_vertical {
                println!("  found cached vertical {min_heat}");
                return Some(min_heat);
            }
        }
    }

    if start.phases[0..start.phases.len() - 1].contains(phase) {
        //println!("----------------- LOOP --------------------");
        return None;
    }

    if let Some(max_heat) = start.max_heat {
        if start.heat(grid) > max_heat {
            println!(
                "-----Heat TOO BIG----------{} {}",
                start.heat(grid),
                max_heat
            );
            return None;
        }
    }

    if phase.position == grid.bottom_right() {
        println!("SOLUTION {}", start.heat(grid));
        return Some(0);
    }

    //let mut min_heat = start.max_heat;
    let mut min_heat: Option<u32> = None;
    for direction in phase.direction.orthogonal() {
        let mut new_positions = vec![];
        for position in phase.position.steps(3, direction, grid) {
            //println!("---->> Position: {position:?}");
            let mut new_path = start.clone();
            new_path.max_heat = min_heat.map(|min_heat| new_path.heat(grid) + min_heat);
            new_positions.push(position);
            new_path.positions.extend(new_positions.iter());
            new_path.phases.push(Phase {
                position,
                direction,
            });

            if let Some(mut next_min_heat) = find_min_path(new_path, grid) {
                next_min_heat += new_positions
                    .iter()
                    .map(|x| grid.cell(*x).inner)
                    .sum::<u32>();
                min_heat = Some(if let Some(old_min_heat) = min_heat {
                    old_min_heat.min(next_min_heat)
                } else {
                    next_min_heat
                });
            }
        }
    }

    if let Some(min_heat) = min_heat {
        match phase.direction.alignment() {
            Alignment::Horizontal => {
                println!("  CACHE horizontal {:?} {min_heat}", phase.position);
                grid.cell_mut(phase.position).visited_horizontal = Some(min_heat);
            }
            Alignment::Vertical => {
                println!("  CACHE vertical {:?} {min_heat}", phase.position);
                grid.cell_mut(phase.position).visited_vertical = Some(min_heat);
            }
        }
    }

    min_heat
}

fn minimum_path(start: Path, grid: &mut Grid<Number>, solutions: &mut Vec<Path>) -> bool {
    // ----- START Debug print
    /*
    use colored::Colorize;
    grid.print(|position: Position, c: char| {
        if !grid.cell(position).solved_horizontal.is_empty()
            || !grid.cell(position).solved_vertical.is_empty()
        {
            //c.to_string().green().reversed().to_string()
            "X".into()
        } else if start.positions.iter().any(|p| *p == position) {
            //c.to_string().red().to_string()
            ".".into()
        } else {
            c.to_string()
        }
    });
    println!();
    */
    // ----- END Debug print

    let phase = start.phases.last().unwrap();

    // No loops.
    /*
        if start.phases[0..start.phases.len() - 1].contains(phase) {
    //        println!("----------------- LOOP --------------------");
            return false;
        }
        */

    //println!("{phase:?}");
    //println!("-->> {}", start.heat(grid));

    //if start.heat(grid) > 1561 {
    //if start.heat(grid) > 3000 {
    //if start.heat(grid) > 2000 {
    if start.heat(grid) > 111 {
        //if start.heat(grid) > 333 {
        return false;
    }

    if phase.position == grid.bottom_right() {
        //        println!("found solution!!!!!!!!! heat:{}", start.heat(grid));

        /*
        // Store minimum solutions.
        if solutions.is_empty() {
            *solutions = vec![start.clone()];
        } else {
            match solutions[0].heat(grid).cmp(&start.heat(grid)) {
                std::cmp::Ordering::Equal => solutions.push(start.clone()),
                std::cmp::Ordering::Greater => *solutions = vec![start.clone()],
                _ => {}
            }
        }
        */

        // Store shortest paths from end.
        let cell = grid.cell_mut(grid.bottom_right());
        match phase.direction.alignment() {
            Alignment::Horizontal => {
                //                println!(" - solution horizontal {:?}", phase);
                if cell.solved_horizontal.is_empty() {
                    cell.solved_horizontal.push(Path {
                        phases: vec![],
                        positions: vec![],
                        finished: false,
                        max_heat: None,
                    });
                }
            }
            Alignment::Vertical => {
                //                println!(" - solution vertical {:?}", phase);
                if cell.solved_vertical.is_empty() {
                    cell.solved_vertical.push(Path {
                        phases: vec![],
                        positions: vec![],
                        finished: false,
                        max_heat: None,
                    });
                }
            }
        }

        return true;
    }

    // Store minimum heat to each phase location and cull out larger heats.
    match phase.direction.alignment() {
        Alignment::Horizontal => {
            if let Some(heat) = grid.cell(phase.position).visited_horizontal {
                match heat.cmp(&start.heat(grid)) {
                    std::cmp::Ordering::Less => return false,
                    std::cmp::Ordering::Greater => {
                        grid.cell_mut(phase.position).visited_horizontal = Some(start.heat(grid));
                    }
                    _ => {}
                }
            } else {
                grid.cell_mut(phase.position).visited_horizontal = Some(start.heat(grid));
            }
        }
        Alignment::Vertical => {
            if let Some(heat) = grid.cell(phase.position).visited_vertical {
                match heat.cmp(&start.heat(grid)) {
                    std::cmp::Ordering::Less => return false,
                    std::cmp::Ordering::Greater => {
                        grid.cell_mut(phase.position).visited_vertical = Some(start.heat(grid));
                    }
                    _ => {}
                }
            } else {
                grid.cell_mut(phase.position).visited_vertical = Some(start.heat(grid));
            }
        }
    }

    // Check for cached minimum path to end.
    match phase.direction.alignment() {
        Alignment::Horizontal => {
            if !grid.cell(phase.position).solved_horizontal.is_empty() {
                //                println!("__________cached horizontal");
                return true;
            }
        }
        Alignment::Vertical => {
            if !grid.cell(phase.position).solved_vertical.is_empty() {
                //                println!("__________cached vertical");
                return true;
            }
        }
    }

    let mut found_one = false;

    for direction in phase.direction.orthogonal() {
        //        println!("____ try direction: {direction:?}");
        let mut new_positions = Vec::new();
        let mut new_cached_paths = Vec::new();

        for position in phase.position.steps(3, direction, grid) {
            //            println!("____ try position: {position:?}");
            // Calculate extra path traveled.
            let mut new_path = start.clone();
            new_positions.push(position);
            new_path.positions.extend(new_positions.iter());
            let new_phase = Phase {
                position,
                direction,
            };
            new_path.phases.push(new_phase);

            if minimum_path(new_path, grid, solutions) {
                found_one = true;
                //                println!("alignment:{:?}", direction.alignment());
                let old_cached_paths = match direction.alignment() {
                    Alignment::Horizontal => &grid.cell(position).solved_horizontal,
                    Alignment::Vertical => &grid.cell(position).solved_vertical,
                };
                //                println!("old_cached_paths: {:?}", old_cached_paths);
                for cached in old_cached_paths {
                    let mut cached_clone = cached.clone();
                    cached_clone.phases.push(new_phase);
                    cached_clone.positions.extend(new_positions.iter());
                    new_cached_paths.push(cached_clone);
                }
                //                println!("new_cached_paths: {:?}", new_cached_paths);
            }
        }

        for cached_path in new_cached_paths {
            match direction.alignment() {
                Alignment::Horizontal => {
                    if grid.cell(phase.position).solved_horizontal.is_empty() {
                        //                        println!("horizontal is empty in {:?}: {:?}", phase.position, cached_path);
                        grid.cell_mut(phase.position)
                            .solved_horizontal
                            .push(cached_path);
                    } else {
                        //                        println!("horizontal is not empty");
                        match cached_path
                            .heat(grid)
                            .cmp(&grid.cell(phase.position).solved_horizontal[0].heat(grid))
                        {
                            std::cmp::Ordering::Less => {
                                grid.cell_mut(phase.position).solved_horizontal = vec![cached_path]
                            }
                            std::cmp::Ordering::Equal => grid
                                .cell_mut(phase.position)
                                .solved_horizontal
                                .push(cached_path),
                            std::cmp::Ordering::Greater => {}
                        }
                    }
                }
                Alignment::Vertical => {
                    if grid.cell(phase.position).solved_vertical.is_empty() {
                        //                        println!("vertical is empty in {:?}: {:?}", phase.position, cached_path);
                        grid.cell_mut(phase.position)
                            .solved_vertical
                            .push(cached_path);
                    } else {
                        //                        println!("vertical is not empty");
                        match cached_path
                            .heat(grid)
                            .cmp(&grid.cell(phase.position).solved_vertical[0].heat(grid))
                        {
                            std::cmp::Ordering::Less => {
                                grid.cell_mut(phase.position).solved_vertical = vec![cached_path]
                            }
                            std::cmp::Ordering::Equal => grid
                                .cell_mut(phase.position)
                                .solved_vertical
                                .push(cached_path),
                            std::cmp::Ordering::Greater => {}
                        }
                    }
                }
            }
        }
    }

    found_one
}

fn min_path_wide(mut paths: Vec<Path>, grid: &mut Grid<Number>) -> Vec<Path> {
    while paths
        .iter()
        .any(|x| x.phases[0].position != grid.bottom_right())
    {
        let mut added = Vec::new();
        for path in paths.iter_mut().filter(|x| !x.finished) {
            added.append(&mut path.step(grid));
        }

        let mut new_grid_heats = Vec::new();
        added.retain(|x| {
            if x.heat(grid) > 111 {
                println!("too big");
                return false;
            }
            let last_phase = *x.phases.last().unwrap();
            match last_phase.direction.alignment() {
                Alignment::Horizontal => {
                    if let Some(heat) = grid.cell(last_phase.position).visited_horizontal {
                        x.heat(grid) <= heat
                    } else {
                        grid.cell_mut(last_phase.position).visited_horizontal = Some(x.heat(grid));
                        new_grid_heats.push(last_phase);
                        true
                    }
                }
                Alignment::Vertical => {
                    if let Some(heat) = grid.cell(last_phase.position).visited_vertical {
                        x.heat(grid) <= heat
                    } else {
                        grid.cell_mut(last_phase.position).visited_vertical = Some(x.heat(grid));
                        new_grid_heats.push(last_phase);
                        true
                    }
                }
            }
        });

        /*
        added.retain(|added| {
            added.heat(grid) < 111
        });
        added.retain(|added| {
            !paths.iter().any(|path| {
                path.phases
                    .iter()
                    .any(|phase| phase == added.phases.last().unwrap())
            })
        });
        */
        //added.retain(|new| {
        //    !new.phases[0..new.phases.len() - 1].contains(new.phases.last().unwrap())
        //});
        //println!("{:?}", &added[0..10.min(added.len())]);
        println!("{} {}", added.len(), paths.len());
        paths = added;
    }

    paths
}

impl Path {
    fn heat(&self, grid: &Grid<Number>) -> u32 {
        self.positions.iter().map(|x| grid.cell(*x).inner).sum()
    }

    fn step(&mut self, grid: &Grid<Number>) -> Vec<Path> {
        let mut new_paths = Vec::new();
        for direction in self.phases.last().unwrap().direction.orthogonal() {
            let mut new_positions = Vec::new();
            for position in self
                .phases
                .last()
                .unwrap()
                .position
                .steps(3, direction, grid)
            {
                let finished = position == grid.bottom_right();
                new_positions.push(position);
                let new_phase = Phase {
                    position,
                    direction,
                };

                let mut old_phases = self.phases.clone();
                old_phases.push(new_phase);
                let new_path = Path {
                    phases: old_phases,
                    positions: new_positions.clone(),
                    finished,
                    max_heat: None,
                };
                new_paths.push(new_path);
            }
        }
        new_paths
    }
}

#[allow(unused)]
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

impl From<char> for Number {
    fn from(value: char) -> Self {
        Number {
            inner: value.to_digit(10).unwrap(),
            visited_horizontal: None,
            visited_vertical: None,
            solved_horizontal: Vec::new(),
            solved_vertical: Vec::new(),
        }
    }
}

impl From<&Number> for char {
    fn from(value: &Number) -> Self {
        value.inner.to_string().chars().collect::<Vec<_>>()[0]
    }
}
