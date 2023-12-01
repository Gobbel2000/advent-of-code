use std::fs;
use std::ops::RangeInclusive;
use std::process::exit;

static INPUT: &str = "input/day4.txt";

fn main() {
    let input = fs::read_to_string(INPUT)
        .expect("Could not open input file");

    let mut count = 0;
    for line in input.lines() {
        let (r1, r2) = parse_line(line);
        if range_contains(&r1, &r2) || range_contains(&r2, &r1) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn parse_line(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let numbers: Vec<i32> = line.split(&['-', ','][..])
        .map(|s| s.parse::<i32>().unwrap_or_else(|e| {
            eprintln!("Malformatted input: {}", e);
            exit(1); }))
        .collect();
    if numbers.len() != 4 {
        eprintln!("Incorrect amount of numbers");
        exit(2);
    }
    return (RangeInclusive::new(numbers[0], numbers[1]),
            RangeInclusive::new(numbers[2], numbers[3]));
}

// Return true if r2 is fully contained within r1
fn range_contains<T>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool
where T: PartialOrd {
    r1.contains(r2.start()) && r1.contains(r2.end())
}
