#![allow(unused)]
use std::{collections::HashMap, io::Write};

use aoc23::{Alignment, Direction, Grid, Position};

fn main() {
    let grid = Grid::<Number>::from(include_str!("test.txt"));
    //println!("{grid}");

    let start = Position::new(0, 0);

    let start_east = Corner {
        position: start,
        alignment: Alignment::Horizontal,
    };
    let start_south = Corner {
        position: start,
        alignment: Alignment::Vertical,
    };

    let east_path = Path {
        positions: vec![],
        corners: vec![start_east],
        step_heat: 0,
    };
    let south_path = Path {
        positions: vec![],
        corners: vec![start_south],
        step_heat: 0,
    };

    /*
    let mut cache = HashMap::new();
    let east_solution = min_path(&east_path, grid.bottom_right(), None, &grid, &mut cache);
    //println!("{east_solution:?}");

    let mut cache = HashMap::new();
    let south_solution = min_path(&south_path, grid.bottom_right(), None, &grid, &mut cache);
    println!("{east_solution:?} {south_solution:?}");
    */

    //let size = grid.dim().r;

    /*
    // ----------------- LATEST ---------------
    let mut cache: HashMap<Corner, u32> = HashMap::new();
    for diagonal in grid.diagonals() {
        for alignment in Alignment::all() {
            for position in diagonal.iter() {
                let path = Path {
                    positions: vec![],
                    corners: vec![Corner {
                        position: *position,
                        alignment,
                    }],
                    step_heat: 0,
                };
                let result =
                    min_path(&path, grid.bottom_right(), None, None, &grid, &mut cache).unwrap();
            }
        }
    }
    println!("{:?} {:?}", cache.get(&start_east), cache.get(&start_south));
    println!(
        "{:?} {:?}",
        cache.get(&Corner {
            position: Position { r: 3, c: 4 },
            alignment: Alignment::Vertical
        }),
        cache.get(&Corner {
            position: Position { r: 3, c: 4 },
            alignment: Alignment::Horizontal
        })
    );
    // ----------------- LATEST ---------------
    */

    let cell_count = grid.dim().r as f32 * grid.dim().c as f32;
    let mut completed_cells = 0f32;

    let mut cache: HashMap<Corner, u32> = HashMap::new();
    let mut sub_cache: HashMap<Corner, u32> = HashMap::new();
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
            /*
            println!("FROM: {from:?} {from_al:?}");

            grid.print(|position: Position, c: char| {
                if cache.keys().any(|x| x.position == position) {
                    "X".into()
                } else {
                    c.to_string()
                }
            });
            println!();
            */

            let mut from_min = match from_al {
                Alignment::Horizontal => {
                    if let Some(cell_to_right) = from.step(Direction::E, &grid) {
                        let mut min = 0;
                        min += *cache
                            .get(&Corner {
                                position: cell_to_right,
                                alignment: Alignment::Vertical,
                            })
                            .unwrap();
                        min += grid.cell(from.step(Direction::E, &grid).unwrap()).inner;
                        min
                    } else {
                        let mut min = 0;
                        min += grid.cell(from.step(Direction::W, &grid).unwrap()).inner;
                        min += grid
                            .cell(
                                from.step(Direction::W, &grid)
                                    .unwrap()
                                    .step(Direction::S, &grid)
                                    .unwrap(),
                            )
                            .inner;
                        min += grid.cell(from.step(Direction::S, &grid).unwrap()).inner;
                        min += *cache
                            .get(&Corner {
                                position: from.step(Direction::S, &grid).unwrap(),
                                alignment: Alignment::Vertical,
                            })
                            .unwrap();
                        //println!("/////H//////>>  return min {min}");
                        min
                    }
                }
                Alignment::Vertical => {
                    if let Some(cell_below) = from.step(Direction::S, &grid) {
                        let mut min = 0;
                        min += *cache
                            .get(&Corner {
                                position: cell_below,
                                alignment: Alignment::Horizontal,
                            })
                            .unwrap();
                        min += grid.cell(from.step(Direction::S, &grid).unwrap()).inner;
                        min
                    } else {
                        let mut min = 0;
                        min += grid.cell(from.step(Direction::N, &grid).unwrap()).inner;
                        min += grid
                            .cell(
                                from.step(Direction::N, &grid)
                                    .unwrap()
                                    .step(Direction::E, &grid)
                                    .unwrap(),
                            )
                            .inner;
                        min += grid.cell(from.step(Direction::E, &grid).unwrap()).inner;
                        min += *cache
                            .get(&Corner {
                                position: from.step(Direction::E, &grid).unwrap(),
                                alignment: Alignment::Horizontal,
                            })
                            .unwrap();
                        //println!("/////V//////>>  return min {min}");
                        min
                    }
                }
            };
            //println!("=0000000=======>>>> {from_min}");
            for (to, to_al) in bottom
                .iter()
                .flat_map(|x| [(x, Alignment::Vertical), (x, Alignment::Horizontal)])
            {
                //println!("TO: {to:?} {to_al:?}");
                let mut to_min = *cache
                    .get(&Corner {
                        position: *to,
                        alignment: to_al,
                    })
                    .unwrap();
                let min_heat_to = from.r.abs_diff(to.r) + from.c.abs_diff(to.c);
                to_min += min_heat_to as u32;

                if to_min < from_min {
                    let allowed_heat = from_min - to_min;
                    //if allowed_heat > 100 {
                    //    println!("=====>>>>>> ALLOWED HEAT {allowed_heat}");
                    //}
                    std::io::stdout().flush();
                    //sub_cache.clear();
                    if let Some(min) = min_path(
                        &Path {
                            positions: vec![],
                            corners: vec![Corner {
                                position: *from,
                                alignment: from_al,
                            }],
                            step_heat: 0,
                        },
                        *to,
                        Some(to_al),
                        Some((0, allowed_heat)),
                        &grid,
                        //&mut sub_cache, // ???
                        &mut cache, // ???
                    ) {
                        //println!("-- min_path returned {min}");
                        from_min = from_min.min(min);
                    }
                }

                //println!("top: {from:?} {from_al:?}");
                //println!("destination: {to:?} {to_al:?}");
            }
            //println!("===1111111=====>>>> {from_min}");
            completed_cells += 0.5;
            //println!("{:3.2} ALLOW", completed_cells * 100f32 / cell_count);
            cache.insert(
                Corner {
                    position: *from,
                    alignment: from_al,
                },
                from_min,
            );
        }
    }
    println!("{:?} {:?}", cache.get(&start_east), cache.get(&start_south));

    /*
    println!(
        "{:?} {:?}",
        cache.get(&Corner {
            position: Position { r: 0, c: 1 },
            alignment: Alignment::Vertical
        }),
        cache.get(&Corner {
            position: Position { r: 0, c: 1 },
            alignment: Alignment::Horizontal
        })
    );
    */

    /*
    for i in 0..size {
        for (i_row, i_column) in (size - i..size).rev().zip(size - i..size) {
            println!("{i_row},{i_column}");

            for alignment in Alignment::all() {
                let start = Position::new(i_row, i_column);
                let path = Path {
                    positions: vec![],
                    corners: vec![Corner {
                        position: start,
                        alignment,
                    }],
                    step_heat: 0,
                };
                let result = min_path(&path, grid.bottom_right(), None, &grid, &mut cache).unwrap();
                //println!("000 {start} {} {:?}", result, alignment);
            }
        }
    }

    for i in (0..size).rev() {
        for (i_row, i_column) in (0..i).zip((0..i).rev()) {
            println!("{i_row},{i_column}");

            for alignment in Alignment::all() {
                let start = Position::new(i_row, i_column);
                let path = Path {
                    positions: vec![],
                    corners: vec![Corner {
                        position: start,
                        alignment,
                    }],
                    step_heat: 0,
                };
                let result = min_path(&path, grid.bottom_right(), None, &grid, &mut cache).unwrap();
                //println!("000 {start} {} {:?}", result, alignment);
            }
        }
    }
    */

    /*
    // Assume square grid.
    let size = grid.dim().r;
    let mut cache = HashMap::new();
    for i in (0..size).rev() {
        for x in (i..size).rev() {
            let i_row = x;
            let i_column = i;

            for alignment in Alignment::all() {
                let start = Position::new(i_row, i_column);
                let path = Path {
                    positions: vec![],
                    corners: vec![Corner {
                        position: start,
                        alignment,
                    }],
                    step_heat: 0,
                };
                let result = min_path(&path, None, &grid, &mut cache).unwrap();
                //println!("000 {start} {} {:?}", result, alignment);
            }

            let i_row = i;
            let i_column = x;

            for alignment in Alignment::all() {
                let start = Position::new(i_row, i_column);
                let path = Path {
                    positions: vec![],
                    corners: vec![Corner {
                        position: start,
                        alignment,
                    }],
                    step_heat: 0,
                };
                let result = min_path(&path, None, &grid, &mut cache).unwrap();
                //println!("000 {start} {} {:?}", result, alignment);
            }
        }
    }
    println!("{:?} {:?}", cache.get(&start_east), cache.get(&start_south));
    */

    /*
    let mut cache = HashMap::new();
    for i_row in (0..grid.dim().r).rev() {
        for i_column in (0..grid.dim().c).rev() {
            for alignment in Alignment::all() {
                let start = Position::new(i_row, i_column);
                let path = Path {
                    positions: vec![],
                    corners: vec![Corner {
                        position: start,
                        alignment,
                    }],
                    step_heat: 0,
                };
                let result = min_path(&path, None, &grid, &mut cache).unwrap();
                //println!("000 {start} {} {:?}", result, alignment);
            }
        }
    }

    println!("{:?} {:?}", cache.get(&start_east), cache.get(&start_south));
    */
}

struct RecursionPath {
    path: Path,
    max: u32,
}

fn min_path(
    start: &Path,
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
        if start.heat(grid) - start_heat + min_heat_to as u32 > max_heat {
            //println!("TOO MUCH HEAT");
            return None;
        }
    }

    if start.is_loop() {
        //println!("loop");
        return None;
    }

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
    for path in start.step(3, grid).iter() {
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
                start.heat(grid) + path.step_heat - grid.cell(path.last_corner().position).inner,
                min_heat.unwrap(),
            ));
        }
        // Search the sub-path and keep track of minimum sub-search.
        if let Some(mut solution_heat) =
            min_path(path, end, end_alignment, recursion_max_heat, grid, cache)
        {
            solution_heat += path.step_heat;
            min_heat = Some(match min_heat {
                Some(x) => x.min(solution_heat),
                None => solution_heat,
            });
        }
    }

    if max_heat.is_none() {
        if let Some(min_heat) = min_heat {
            cache
                .entry(start.last_corner())
                .and_modify(|x| *x = (*x).min(min_heat))
                .or_insert(min_heat);
            grid.print(|position: Position, c: char| {
                if cache.keys().any(|x| x.position == position) {
                    "X".into()
                } else if start.positions.iter().any(|p| *p == position) {
                    ".".into()
                } else {
                    c.to_string()
                }
            });
            println!(
                "caching {} for {},{} {:?}",
                cache.get(&start.last_corner()).unwrap(),
                start.last_corner().position.r,
                start.last_corner().position.c,
                start.last_corner().alignment,
            );
            println!();
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
    positions: Vec<Position>,
    corners: Vec<Corner>,
    step_heat: u32,
    //inner_heat: u32,
}

impl Path {
    fn heat(&self, grid: &Grid<Number>) -> u32 {
        self.positions.iter().map(|x| grid.cell(*x).inner).sum()
    }

    fn is_loop(&self) -> bool {
        self.corners[0..self.corners.len() - 1].contains(&self.last_corner())
    }

    fn last_corner(&self) -> Corner {
        *self.corners.last().unwrap()
    }

    fn step(&self, count: usize, grid: &Grid<Number>) -> Vec<Path> {
        let mut new_paths = Vec::new();
        for direction in self.last_corner().alignment.directions() {
            let mut step_positions: Vec<Position> = Vec::new();
            for position in self.last_corner().position.steps(count, direction, grid) {
                step_positions.push(position);
                let mut positions = self.positions.clone();
                positions.extend(step_positions.iter());

                let mut corners = self.corners.clone();
                let step_corner = Corner {
                    position,
                    alignment: self.last_corner().alignment.orthogonal(),
                };
                corners.push(step_corner);

                new_paths.push(Path {
                    positions,
                    corners,
                    step_heat: step_positions.iter().map(|x| grid.cell(*x).inner).sum(),
                });
            }
        }
        new_paths
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Path:");
        writeln!(f, "  step_heat: {}", self.step_heat);
        writeln!(f, "  Positions:");
        for p in &self.positions {
            writeln!(f, "    {p}");
        }
        writeln!(f, "  Corners:");
        for c in &self.corners {
            writeln!(f, "    {} {:?}", c.position, c.alignment);
        }
        writeln!(f)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Number {
    inner: u32,
    min_vertical: Option<u32>,
    min_horizontal: Option<u32>,
}

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
