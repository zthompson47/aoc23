use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("{input}");

    // Import brick data and sort by height of brick bottoms.
    let mut grid: Vec<Brick> = input.lines().map(Into::into).collect();
    grid.sort_by(|a, b| a.lower_end.z.cmp(&b.lower_end.z));
    //print(&grid);

    println!("Part 1: {}", part1(grid.clone()));
}

fn part1(mut grid: Vec<Brick>) -> usize {
    println!("Before:");
    print(&grid);

    // Keep track of which level the tops of bricks land in.
    type ZLevel = usize;
    type BrickId = usize;
    let mut brick_tips: HashMap<ZLevel, Vec<BrickId>> = HashMap::new();
    //let mut tips: Vec<Vec<Brick>> = vec![Default::default(); grid.len()];

    // Drop bricks to ground, starting with brick closest to ground.
    for brick_id in 0..grid.len() {
        let mut j = grid[brick_id].lowest_level() - 1;
        while j > 0 {
            // Look for intersections with bricks whose tip is in this level.
            let mut intersections: Vec<BrickId> = Vec::new();
            if let Some(other_brick_ids) = brick_tips.get(&j) {
                for other_brick_id in other_brick_ids {
                    println!("check intersect: brick_id: {brick_id}, other: {other_brick_id}");
                    if grid[brick_id].intersects(&grid[*other_brick_id]) {
                        intersections.push(*other_brick_id);
                    }
                }
            }
            if !intersections.is_empty() {
                println!("-{brick_id}->> {:?}", intersections);
                if intersections.len() == 1 {
                    grid[intersections[0]].is_support = true;
                }
                break;
            }

            j -= 1;
        }
        let fall = grid[brick_id].lowest_level() - 1 - j;
        grid[brick_id].lower_end.z -= fall;
        grid[brick_id].upper_end.z -= fall;
        brick_tips
            .entry(grid[brick_id].uppermost_level())
            .and_modify(|bricks| bricks.push(brick_id))
            .or_insert(vec![brick_id]);
    }

    println!("After:");
    print(&grid);

    println!("Tips:");
    println!("{brick_tips:?}\n");

    grid.iter().filter(|x| !x.is_support).count()
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let coordinates: Vec<usize> = value
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        assert_eq!(coordinates.len(), 3);
        Position {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    lower_end: Position,
    upper_end: Position,
    is_support: bool,
}

impl Brick {
    fn lowest_level(&self) -> usize {
        self.lower_end.z
    }

    fn uppermost_level(&self) -> usize {
        self.upper_end.z
    }

    fn intersects(&self, other: &Self) -> bool {
        let mut points: HashSet<(usize, usize)> = HashSet::new();
        let mut other_points: HashSet<(usize, usize)> = HashSet::new();

        let range_x = if self.lower_end.x <= self.upper_end.x {
            self.lower_end.x..=self.upper_end.x
        } else {
            self.upper_end.x..=self.lower_end.x
        };
        let range_y = if self.lower_end.y <= self.upper_end.y {
            self.lower_end.y..=self.upper_end.y
        } else {
            self.upper_end.y..=self.lower_end.y
        };
        for x in range_x {
            for y in range_y.clone() {
                points.insert((x, y));
            }
        }

        let range_x = if other.lower_end.x <= other.upper_end.x {
            other.lower_end.x..=other.upper_end.x
        } else {
            other.upper_end.x..=other.lower_end.x
        };
        let range_y = if other.lower_end.y <= other.upper_end.y {
            other.lower_end.y..=other.upper_end.y
        } else {
            other.upper_end.y..=other.lower_end.y
        };
        for x in range_x {
            for y in range_y.clone() {
                other_points.insert((x, y));
            }
        }

        println!("Points: {points:?}");
        println!("OTHER Points: {other_points:?}");

        let intersection: HashSet<&(usize, usize)> = points.intersection(&other_points).collect();
        println!("Intersections: {intersection:?}\n");
        !intersection.is_empty()
    }
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let ends: Vec<Position> = value.split("~").map(Into::into).collect();
        assert_eq!(ends.len(), 2);
        let (lower_end, upper_end) = (ends[0], ends[1]);
        assert!(lower_end.x <= upper_end.x);
        assert!(lower_end.y <= upper_end.y);
        assert!(lower_end.z <= upper_end.z);
        Brick {
            lower_end,
            upper_end,
            is_support: false,
        }
    }
}

impl std::fmt::Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let support = if self.is_support { " S" } else { "" };
        write!(f, "{}~{}{support}", self.lower_end, self.upper_end)
    }
}

fn print(grid: &[Brick]) {
    grid.iter().for_each(|x| println!("{x}"));
    println!();
}
