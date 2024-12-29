fn main() {
    let iter = Box::new((0..10).take(3)) as Box<dyn Iterator<Item = i32>>;

    for i in iter {
        println!("{i}");
    }

    for line in include_str!("asdf.txt").lines().take(2) {
        println!("{line}");
    }
}
