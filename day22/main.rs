fn main() {
    let input = include_str!("test.txt");
    print!("{input}");
    let grid: Vec<Brick> = input.lines().map(Into::into).collect();
    println!("{grid:#?}");
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
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

#[derive(Debug)]
struct Brick {
    lower_end: Position,
    upper_end: Position,
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
        }
    }
}
