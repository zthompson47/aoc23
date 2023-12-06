const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    let input = include_str!("input.txt").lines();
    let mut result_1 = 0;
    let mut result_2 = 0;

    for (i, mut line) in input.enumerate() {
        line = line.split(':').last().unwrap();

        let mut game_possible = true;
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        line.split(';').for_each(|game| {
            game.split(',').for_each(|color_item| {
                let mut color_item = color_item.split_whitespace();

                let num = color_item.next().unwrap().parse::<usize>().unwrap();
                let color = color_item.next().unwrap();

                let color_possible = match color {
                    "red" => {
                        if num > max_red {
                            max_red = num;
                        }
                        num <= MAX_RED
                    }
                    "green" => {
                        if num > max_green {
                            max_green = num;
                        }
                        num <= MAX_GREEN
                    }
                    "blue" => {
                        if num > max_blue {
                            max_blue = num;
                        }
                        num <= MAX_BLUE
                    }
                    wtf => {
                        println!("wtf: {wtf}");
                        panic!()
                    }
                };

                if !color_possible {
                    game_possible = false;
                }
            });
        });

        let power = max_red * max_green * max_blue;
        result_2 += power;

        if game_possible {
            result_1 += i + 1;
        }
    }

    println!("{result_1} {result_2}");
}
