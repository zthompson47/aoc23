fn main() {
    println!(
        "Part 1: {}",
        include_str!("input.txt")
            .split(',')
            .map(hash)
            .sum::<usize>()
    );

    let mut boxes = vec![Vec::<Lens>::new(); 256];
    for step in include_str!("input.txt").trim().split(',') {
        if step.contains('=') {
            // Add or replace lens.
            let (label, focal_length) = step.split_once('=').unwrap();
            let lens = Lens {
                label,
                focal_length: focal_length.parse::<usize>().unwrap(),
            };
            let box_idx = hash(label);
            if let Some(lens_idx) = boxes[box_idx].iter().position(|lens| lens.label == label) {
                boxes[box_idx].remove(lens_idx);
                boxes[box_idx].insert(lens_idx, lens);
            } else {
                boxes[box_idx].push(lens);
            }
        } else if step.ends_with('-') {
            // Remove lens.
            let label = step.strip_suffix('-').unwrap();
            let box_idx = hash(label);
            if let Some(lens_idx) = boxes[box_idx].iter().position(|lens| lens.label == label) {
                boxes[box_idx].remove(lens_idx);
            }
        } else {
            unreachable!()
        }
    }

    println!(
        "Part 2: {}",
        boxes
            .iter()
            .enumerate()
            .map(|(i, boxx)| {
                boxx.iter()
                    .enumerate()
                    .map(|(j, lens)| (i + 1) * (j + 1) * lens.focal_length)
                    .sum::<usize>()
            })
            .sum::<usize>()
    );
}

#[derive(Clone, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: usize,
}

fn hash(seq: &str) -> usize {
    seq.trim().as_bytes().iter().fold(0, |mut acc, c| {
        let c = *c as usize;
        acc += c;
        acc *= 17;
        acc %= 256;
        acc
    })
}
