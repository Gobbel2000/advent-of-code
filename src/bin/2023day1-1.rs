use std::fs;
use std::collections::HashMap;

use regex::Regex;

static INPUT: &str = "input/2023day1.txt";

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();    
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
            let first = u32::from_str_radix(d, 10).unwrap_or_else(|_| digits[d]);

            let last_match = re_last.captures(l).unwrap();
            let d = last_match.get(1).unwrap().as_str();
            let last = u32::from_str_radix(d, 10).unwrap_or_else(|_| digits[d]);
            first * 10 + last
        })
        .sum();
    println!("{number}");
}
