use std::fs;

static INPUT: &str = "input/day1.txt";

fn main() {
    let input: String = fs::read_to_string(INPUT)
        .expect("Couldn't open input file");
    let mut max: u32 = 0;
    let mut cur: u32 = 0;

    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(item) => cur += item,
            Err(_) => {  // Encountered newline
                if cur > max {
                    max = cur;
                }
                cur = 0;
            }
        }
    }
    println!("{}", max);
}
