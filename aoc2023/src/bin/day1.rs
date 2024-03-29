use std::collections::HashMap;

use regex::Regex;

fn part1(input: String) {
    let number: u32 = input.lines()
        .map(|l| {
            let mut digits = l.chars()
                .filter_map(|c| c.to_digit(10));
            let first = digits.next().expect("Each line should have at least one digit");
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum();
    println!("{number}");
}

fn part2(input: String) {
    let re_first = Regex::new(r"\d|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine)").unwrap();
    // Read any character greedily first to find the last digit
    let re_last = Regex::new(r".*(\d|(one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine))").unwrap();
    let digits = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let number: u32 = input.lines()
        .map(|l| {
            let d = re_first.find(l).unwrap().as_str();
            let first = d.parse::<u32>().unwrap_or_else(|_| digits[d]);

            let last_match = re_last.captures(l).unwrap();
            let d = last_match.get(1).unwrap().as_str();
            let last = d.parse::<u32>().unwrap_or_else(|_| digits[d]);
            first * 10 + last
        })
        .sum();
    println!("{number}");
}

util::aoc_main!("day1.txt");
