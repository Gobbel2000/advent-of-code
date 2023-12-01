use std::fs;

static INPUT: &str = "input/2023day1.txt";

fn main() {
    let input = fs::read_to_string(INPUT).unwrap();    
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
