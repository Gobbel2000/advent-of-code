use std::fs;

static INPUT: &str = "input/day1.txt";

fn main() {
    let input: String = fs::read_to_string(INPUT)
        .expect("Couldn't open input file");
    let mut cur: u32 = 0;
    let mut calories: Vec<u32> = Vec::new();

    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(item) => cur += item,
            Err(_) => {  // Encountered newline
                calories.push(cur);
                cur = 0;
            }
        }
    }
    calories.sort_unstable();
    let max3: u32 = calories.pop().unwrap() +
                    calories.pop().unwrap() +
                    calories.pop().unwrap();

    println!("{}", max3);
}
