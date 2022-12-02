mod day1;
mod util;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    print_one("day1::part1", day1::part1);
    print_one("day1::part2", day1::part2);
    println!("Total elapsed time: {:?}", Instant::elapsed(&start));
}

fn print_one(name: &str, exercise: fn() -> i64) {
    let now = Instant::now();

    let result = exercise();

    println!(
        "Elapsed time for {}: {:?}, result: {}",
        name,
        Instant::elapsed(&now),
        result
    );
}
