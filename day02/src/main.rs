const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() {
    let input = include_str!("input.txt").lines();
    let mut result = 0;

    for (i, mut line) in input.enumerate() {
        line = line.split(':').last().unwrap();

        let mut game_possible = true;

        line.split(';').for_each(|game| {
            game.split(',').for_each(|color_item| {
                let mut color_item = color_item.split_whitespace();

                let num = color_item.next().unwrap().parse::<usize>().unwrap();
                let color = color_item.next().unwrap();

                let color_possible = match color {
                    "red" => num <= MAX_RED,
                    "green" => num <= MAX_GREEN,
                    "blue" => num <= MAX_BLUE,
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

        if game_possible {
            result += i + 1;
        }
    }

    println!("{result}");
}
