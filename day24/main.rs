use rug::Rational;

fn main() {
    //let input = include_str!("test.txt");
    //let p_range = 7..=27;
    let input = include_str!("input.txt");
    let p_range = 200000000000000i64..=400000000000000i64;

    let stones: Vec<Hailstone> = input.lines().map(Into::into).collect();
    let mut collisions = 0;

    for i in 0..stones.len() - 1 {
        let stone = &stones[i];
        for other_stone in stones.iter().skip(i + 1) {
            //println!("{stone:#?}");
            //println!("slope: {}, y-inter: {}", stone.slope(), stone.y_intercept());

            /*
            println!(
                "\nHailstone A: {}, {}, {} @ {}, {}, {}",
                stone.px, stone.py, stone.pz, stone.vx, stone.vy, stone.vz
            );
            println!(
                "Hailstone B: {}, {}, {} @ {}, {}, {}",
                other_stone.px,
                other_stone.py,
                other_stone.pz,
                other_stone.vx,
                other_stone.vy,
                other_stone.vz
            );
            */

            let numerator_x = other_stone.y_intercept() - stone.y_intercept();
            let denominator_x = stone.slope() - other_stone.slope();
            if denominator_x != 0 {
                let x = numerator_x / denominator_x;
                //println!("22>> {x}");
                let forward_in_time =
                    { (x < stone.px && stone.vx < 0) || (x > stone.px && stone.vx > 0) }; // maybe check equal?
                if p_range.contains(&x) && forward_in_time {
                    let stone_y = stone.slope() * x.clone() + stone.y_intercept();
                    let other_stone_y = other_stone.slope() * x + other_stone.y_intercept();
                    //println!("00>> {x} {stone_y} {other_stone_y}");
                    let forward_in_time = {
                        (other_stone_y < other_stone.py && other_stone.vy < 0)
                            || (other_stone_y > other_stone.py && other_stone.vy > 0)
                    }; // maybe check equal?
                   //let forward_in_time = true;
                    if p_range.contains(&stone_y)
                        && stone_y == other_stone_y
                        && forward_in_time
                    {
                        //println!("11>> {x} {stone_y}");
                        collisions += 1;
                    }
                }
            }
        }
    }

    println!("Part 1: {collisions}");
}

//fn approx_eq(f1: f64, f2: f64) -> bool {
//    let factor = 1f64;
//    (f1 * factor).trunc() == (f2 * factor).trunc()
//}

#[derive(Debug)]
struct Hailstone {
    px: i64,
    py: i64,
    #[allow(unused)]
    pz: i64,
    vx: i64,
    vy: i64,
    #[allow(unused)]
    vz: i64,
}

impl Hailstone {
    fn y_intercept(&self) -> Rational {
        self.py - self.slope() * self.px
    }

    fn slope(&self) -> Rational {
        assert_ne!(self.vx, 0);
        //println!("??? {} / {} = {}", self.vy, self.vx, self.vy / self.vx);
        Rational::from((self.vy, self.vx))
    }
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (p, v) = value.split_once(" @ ").unwrap();
        let mut p = p.split(',').map(|x| x.trim().parse::<i64>().unwrap());
        let mut v = v.split(',').map(|x| x.trim().parse::<i64>().unwrap());
        Hailstone {
            px: p.next().unwrap(),
            py: p.next().unwrap(),
            pz: p.next().unwrap(),
            vx: v.next().unwrap(),
            vy: v.next().unwrap(),
            vz: v.next().unwrap(),
        }
    }
}
