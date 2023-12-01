use std::fs;
use std::ops::RangeInclusive;
use std::process::exit;
use std::cmp;

static INPUT: &str = "input/day4.txt";

fn main() {
    let input = fs::read_to_string(INPUT)
        .expect("Could not open input file");

    let mut count = 0;
    for line in input.lines() {
        let (r1, r2) = parse_line(line);
        if ranges_overlap(&r1, &r2) {
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
            RangeInclusive::new(numbers[2], numbers[3]))
}

// Return true r1 and r1 have some overlapping value
fn ranges_overlap<T>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool
where T: Ord {
    let lower = cmp::max(r1.start(), r2.start());
    let upper = cmp::min(r1.end(), r2.end());
    return upper >= lower;
}
