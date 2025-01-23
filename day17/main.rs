use std::{collections::HashMap, io::Write};

use log::{debug, log_enabled, Level};

use aoc23::{Alignment, Direction, Grid, Position};

fn main() {
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .target(env_logger::Target::Stdout)
        .init();
    let mut grid = Grid::<Number>::from(include_str!("input.txt"));
    println!("Part 1: {}", part1(&mut grid));
}

fn part1(grid: &mut Grid<Number>) -> u32 {
    let east_path = Path {
        last_corner: Corner {
            position: Position::new(0, 0),
            alignment: Alignment::Horizontal,
        },
        inner_heat: 0,
        step_heat: 0,
    };
    let south_path = Path {
        last_corner: Corner {
            position: Position::new(0, 0),
            alignment: Alignment::Vertical,
        },
        inner_heat: 0,
        step_heat: 0,
    };

    let mut cache = HashMap::new();
    find_min_path(east_path, grid, &mut cache);
    find_min_path(south_path, grid, &mut cache);

    let cache_vertical = cache.get(&Corner {
        position: grid.bottom_right(),
        alignment: Alignment::Vertical,
    });
    let cache_horizontal = cache.get(&Corner {
        position: grid.bottom_right(),
        alignment: Alignment::Horizontal,
    });

    *cache_vertical.min(cache_horizontal).unwrap()
}

fn min_solution_heat(grid: &Grid<Number>, cache: &HashMap<Corner, u32>) -> Option<u32> {
    let end_vertical = Corner {
        position: grid.bottom_right(),
        alignment: Alignment::Vertical,
    };
    let end_horizontal = Corner {
        position: grid.bottom_right(),
        alignment: Alignment::Horizontal,
    };
    cache
        .get(&end_vertical)
        .min(cache.get(&end_horizontal))
        .copied()
}

fn find_min_path(start: Path, grid: &Grid<Number>, cache: &mut HashMap<Corner, u32>) {
    // Stop paths with more heat than current solution.
    if let Some(min_heat) = min_solution_heat(grid, cache) {
        if start.inner_heat > min_heat {
            return;
        }
    }

    // Stop paths with more heat than cached heat on this corner.
    if let Some(cached_min) = cache.get(&start.last_corner) {
        if start.inner_heat >= *cached_min {
            return;
        }
    }
    cache.insert(start.last_corner, start.inner_heat);

    if start.last_corner.position == grid.bottom_right() {
        debug!("got solution heat: {}", start.inner_heat);
    }

    for path in start.step(3, grid) {
        find_min_path(path, grid, cache);
    }
}

fn part1_diagonals(grid: &Grid<Number>) -> u32 {
    let cell_count = grid.dim().r as f32 * grid.dim().c as f32;
    let mut completed_cells = 0f32;

    let mut cache: HashMap<Corner, u32> = HashMap::new();
    for alignment in Alignment::all() {
        cache.insert(
            Corner {
                position: grid.bottom_right(),
                alignment,
            },
            0,
        );
    }
    for (top, bottom) in grid.diagonals().skip(1).zip(grid.diagonals()) {
        for (from, from_al) in top
            .iter()
            .flat_map(|x| [(x, Alignment::Vertical), (x, Alignment::Horizontal)])
        {
            debug!("___________________________________________________");
            if log_enabled!(Level::Debug) {
                for alignment in Alignment::all() {
                    println!("====={alignment:?}====");
                    grid.print(|position: Position, c: char| {
                        if let Some(x) = cache.get(&Corner {
                            position,
                            alignment,
                        }) {
                            let x = x + grid.cell(position).inner;
                            format!("{:<5}", format!("[{x}]"))
                        } else if position == *from {
                            format!("{:<5}", format!("-{c}-"))
                        } else {
                            format!("{:<5}", format!(" {c} "))
                        }
                    });
                }
                println!();
            }

            let mut from_min = match from_al {
                Alignment::Horizontal => {
                    if let Some(min) = min_cached_heat_from(*from, Direction::E, 3, grid, &cache) {
                        min
                    } else {
                        let mut min = 0;
                        min += grid.cell(from.step(Direction::W, grid).unwrap()).inner;
                        min += grid
                            .cell(
                                from.step(Direction::W, grid)
                                    .unwrap()
                                    .step(Direction::S, grid)
                                    .unwrap(),
                            )
                            .inner;
                        min += grid.cell(from.step(Direction::S, grid).unwrap()).inner;
                        min += *cache
                            .get(&Corner {
                                position: from.step(Direction::S, grid).unwrap(),
                                alignment: Alignment::Vertical,
                            })
                            .unwrap();
                        min
                    }
                }
                Alignment::Vertical => {
                    if let Some(min) = min_cached_heat_from(*from, Direction::S, 3, grid, &cache) {
                        min
                    } else {
                        let mut min = 0;
                        min += grid.cell(from.step(Direction::N, grid).unwrap()).inner;
                        min += grid
                            .cell(
                                from.step(Direction::N, grid)
                                    .unwrap()
                                    .step(Direction::E, grid)
                                    .unwrap(),
                            )
                            .inner;
                        min += grid.cell(from.step(Direction::E, grid).unwrap()).inner;
                        min += *cache
                            .get(&Corner {
                                position: from.step(Direction::E, grid).unwrap(),
                                alignment: Alignment::Horizontal,
                            })
                            .unwrap();
                        min
                    }
                }
            };
            debug!("FROM: {from:?} {from_al:?}");
            debug!("from_min: {from_min}");

            let mut to_min: Option<u32> = None;

            let cache_wall = cache.clone();
            for (to, to_al) in bottom
                .iter()
                .flat_map(|x| [(x, Alignment::Vertical), (x, Alignment::Horizontal)])
            {
                debug!("TO: {to:?}");

                let mut cached_min = *cache
                    .get(&Corner {
                        position: *to,
                        alignment: to_al,
                    })
                    .unwrap();
                let min_heat_to = (from.r.abs_diff(to.r) + from.c.abs_diff(to.c) - 1) as u32;

                if cached_min + min_heat_to < from_min {
                    let allowed_heat = from_min - cached_min;
                    debug!("ALOWED_HEAT: {allowed_heat}");

                    if let Some(possible_min) = min_cached_path_through(
                        Path {
                            inner_heat: 0,
                            last_corner: Corner {
                                position: *from,
                                alignment: from_al,
                            },
                            step_heat: 0,
                        },
                        *to,
                        //grid.bottom_right(),
                        Some((0, allowed_heat)),
                        &grid,
                        &cache_wall,
                    ) {
                        to_min = Some(match to_min {
                            Some(to_min) => to_min.min(possible_min),
                            None => possible_min,
                        });
                        //println!("=====>>>>>>>>  !! to_min:{to_min:?}");
                    }
                }
            }

            let to_min = match to_min {
                Some(to_min) => to_min.min(from_min),
                None => from_min,
            };

            cache.insert(
                Corner {
                    position: *from,
                    alignment: from_al,
                },
                to_min,
            );

            completed_cells += 0.5;
            println!("{:3.2}% ALLOW", completed_cells * 100f32 / cell_count);
        }
    }

    let start = Position::new(0, 0);
    let start_east = Corner {
        position: start,
        alignment: Alignment::Horizontal,
    };
    let start_south = Corner {
        position: start,
        alignment: Alignment::Vertical,
    };

    *cache.get(&start_east).min(cache.get(&start_south)).unwrap()
}

fn min_cached_heat_from(
    from: Position,
    direction: Direction,
    steps: usize,
    grid: &Grid<Number>,
    cache: &HashMap<Corner, u32>,
) -> Option<u32> {
    let mut result: Option<u32> = None;
    let mut step_heat = 0;
    for _ in 0..steps {
        if let Some(position) = from.step(direction, grid) {
            if let Some(cached_heat) = cache.get(&Corner {
                position,
                alignment: direction.alignment().orthogonal(),
            }) {
                step_heat += grid.cell(position).inner;
                result = Some(match result {
                    Some(x) => x.min(cached_heat + step_heat),
                    None => cached_heat + step_heat,
                });
            }
        } else {
            break;
        }
    }
    result
}

fn min_cached_path_through(
    start: Path,
    end: Position,
    max_heat: Option<(u32, u32)>,
    grid: &Grid<Number>,
    cache: &HashMap<Corner, u32>,
) -> Option<u32> {
    //if cache.get(&start.last_corner()).is_some() {
    //    return None;
    //}
    if let Some(val) = cache.get(&start.last_corner()) {
        return Some(*val);
    }

    if let Some((start_heat, max_heat)) = max_heat {
        let min_heat_to = start.last_corner().position.r.abs_diff(end.r)
            + start.last_corner().position.c.abs_diff(end.c);
        if start.heat(grid) - start_heat + min_heat_to as u32 > max_heat {
            debug!(
                "TOO MUCH HEAT start.heat:{} - start_heat:{} + min_heat_to:{} > max_heat:{}",
                start.heat(grid),
                start_heat,
                min_heat_to,
                max_heat
            );
            return None;
        }
    }

    let mut min_heat: Option<u32> = None;
    let mut recursion_max_heat = max_heat;
    let start_heat = start.heat(grid);

    for path in start.step(3, grid) {
        if max_heat.is_none() && min_heat.is_some() {
            recursion_max_heat = Some((
                start_heat + path.step_heat - grid.cell(path.last_corner().position).inner,
                min_heat.unwrap(),
            ));
        }
        // Search the sub-path and keep track of minimum sub-search.
        let path_step_heat = path.step_heat;
        if let Some(mut solution_heat) =
            min_cached_path_through(path, end, recursion_max_heat, grid, cache)
        {
            solution_heat += path_step_heat;
            min_heat = Some(match min_heat {
                Some(x) => x.min(solution_heat),
                None => solution_heat,
            });
        }
    }

    //println!("min_path RETURN");
    min_heat
}

struct RecursionPath {
    path: Path,
    max: u32,
}

fn min_path(
    start: Path,
    end: Position,
    end_alignment: Option<Alignment>,
    max_heat: Option<(u32, u32)>,
    grid: &Grid<Number>,
    cache: &mut HashMap<Corner, u32>,
) -> Option<u32> {
    //println!("min_path({:?}, {end}, {end_alignment:?}, {max_heat:?}", start.last_corner());
    /*
    println!("_______________________________________");
    println!("min_path: {:?}", start.last_corner());
    grid.print(|position: Position, c: char| {
        if cache.keys().any(|x| x.position == position) {
            "X".into()
        } else if start.positions.iter().any(|p| *p == position) {
            ".".into()
        } else {
            c.to_string()
        }
    });
    println!();
    */

    if let Some(min_heat) = cache.get(&start.last_corner()) {
        /*
        println!(
            "---->> got cache {min_heat:?} for {:?}",
            start.last_corner()
        );
        */
        return Some(*min_heat);
    }

    if let Some((start_heat, max_heat)) = max_heat {
        //println!("{} - {} > {}", start.heat(grid), start_heat, max_heat);
        let min_heat_to = start.last_corner().position.r.abs_diff(end.r)
            + start.last_corner().position.c.abs_diff(end.c);

        //if start.heat(grid) - start_heat + min_heat_to as u32 > max_heat {
        if start.heat(grid) - start_heat > max_heat {
            debug!(
                "TOO MUCH HEAT start.heat:{} - start_heat:{} + min_heat_to:{} > max_heat:{}",
                start.heat(grid),
                start_heat,
                min_heat_to,
                max_heat
            );
            return None;
        }
    }

    /*
    if start.is_loop() {
        //println!("loop");
        return None;
    }
    */

    if start.last_corner().position == end {
        //println!("--> solution {}", start.heat(grid));
        if let Some(alignment) = end_alignment {
            if alignment == start.last_corner().alignment {
                return Some(0);
            }
        } else {
            return Some(0);
        }
    }

    let mut min_heat: Option<u32> = None;
    let mut recursion_max_heat = max_heat;
    let mut min_solution_heat: Option<u32> = None;
    /*
    println!("--------STEPS for {}------------", start);
    for p in start.step(3, grid) {
        println!("{p}");
    }
    */
    let start_last_corner = start.last_corner();
    let start_heat = start.heat(grid);
    for path in start.step(3, grid) {
        //.iter() {
        // Try to impose a heat limit on sub-searches.
        if max_heat.is_none() && min_heat.is_some() {
            /*
            println!(
                "MAX_HEAT: recursion_max_heat = start:{} + step_heat:{}, min_heat:{}",
                start.heat(grid),
                path.step_heat,
                min_heat.unwrap()
            );
            */
            //recursion_max_heat = Some((start.heat(grid) + path.step_heat, min_heat.unwrap()));
            recursion_max_heat = Some((
                //start.heat(grid) + path.step_heat - grid.cell(path.last_corner().position).inner,
                start_heat + path.step_heat - grid.cell(path.last_corner().position).inner,
                min_heat.unwrap(),
            ));
        }
        // Search the sub-path and keep track of minimum sub-search.
        let path_step_heat = path.step_heat;
        if let Some(mut solution_heat) =
            min_path(path, end, end_alignment, recursion_max_heat, grid, cache)
        {
            //solution_heat += path.step_heat;
            solution_heat += path_step_heat;
            min_heat = Some(match min_heat {
                Some(x) => x.min(solution_heat),
                None => solution_heat,
            });
        }
    }

    if max_heat.is_none() {
        if let Some(min_heat) = min_heat {
            cache
                //.entry(start.last_corner())
                .entry(start_last_corner)
                .and_modify(|x| *x = (*x).min(min_heat))
                .or_insert(min_heat);
            /*
            grid.print(|position: Position, c: char| {
                if cache.keys().any(|x| x.position == position) {
                    "X".into()
                } else if start.positions.iter().any(|p| *p == position) {
                    ".".into()
                } else {
                    c.to_string()
                }
            });
            */
            /*
            println!(
                "caching {} for {},{} {:?}",
                //cache.get(&start.last_corner()).unwrap(),
                cache.get(&start_last_corner).unwrap(),
                start_last_corner.position.r,
                start_last_corner.position.c,
                start_last_corner.alignment,
            );
            println!();
            */
        }
    }

    //println!("min_path RETURN");
    min_heat
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Corner {
    position: Position,
    alignment: Alignment,
}

#[derive(Debug)]
struct Path {
    last_corner: Corner,
    step_heat: u32,
    inner_heat: u32,
}

impl Path {
    fn heat(&self, grid: &Grid<Number>) -> u32 {
        self.inner_heat
    }

    fn last_corner(&self) -> Corner {
        self.last_corner
    }

    fn step(self, count: usize, grid: &Grid<Number>) -> impl Iterator<Item = Path> + use<'_> {
        let directions = self.last_corner().alignment.directions();
        let alignment = self.last_corner().alignment.orthogonal();

        let inner_heat = self.inner_heat;
        let mut step_heat = 0;
        let first = self
            .last_corner()
            .position
            .steps(count, directions[0], grid)
            .into_iter()
            .map(move |position| {
                step_heat += grid.cell(position).inner;
                let step_corner = Corner {
                    position,
                    //alignment: self.last_corner().alignment.orthogonal(),
                    alignment,
                };
                Path {
                    inner_heat: inner_heat + step_heat,
                    last_corner: step_corner,
                    step_heat,
                }
            });

        let inner_heat = self.inner_heat;
        let mut step_heat = 0;
        let second = self
            .last_corner()
            .position
            .steps(count, directions[1], grid)
            .into_iter()
            .map(move |position| {
                step_heat += grid.cell(position).inner;
                let step_corner = Corner {
                    position,
                    //alignment: self.last_corner().alignment.orthogonal(),
                    alignment,
                };
                Path {
                    inner_heat: inner_heat + step_heat,
                    last_corner: step_corner,
                    step_heat,
                }
            });

        first.chain(second)
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Path:")?;
        writeln!(f, "  step_heat: {}", self.step_heat)?;
        writeln!(f, "  heat: {}", self.inner_heat)?;
        writeln!(f, "  last_corner: {:?}", self.last_corner)?;
        writeln!(f)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Number {
    inner: u32,
    min_vertical: Option<u32>,
    min_horizontal: Option<u32>,
}

/*
impl Number {
    fn cached_min(&self, alignment: Alignment) -> Option<u32> {
        match alignment {
            Alignment::Horizontal => self.min_horizontal,
            Alignment::Vertical => self.min_vertical,
        }
    }

    fn cache_min(&mut self, alignment: Alignment, min: u32) {
        match alignment {
            Alignment::Horizontal => self.min_horizontal = Some(min),
            Alignment::Vertical => self.min_vertical = Some(min),
        }
    }
}
*/

impl From<char> for Number {
    fn from(value: char) -> Self {
        Number {
            inner: value.to_digit(10).unwrap(),
            ..Default::default()
        }
    }
}

impl From<&Number> for char {
    fn from(value: &Number) -> Self {
        value.inner.to_string().chars().collect::<Vec<_>>()[0]
    }
}
