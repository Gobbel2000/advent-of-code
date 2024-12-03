use regex::{Regex, Captures};

fn mul_cap(cap: Captures) -> i32 {
    let a = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let b = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
    a * b
}

fn part1(input: String) {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let res = re.captures_iter(&input).map(mul_cap).sum::<i32>();
    println!("{res}");
}

fn part2(input: String) {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut enabled = true;
    let mut res = 0;
    for cap in re.captures_iter(&input) {
        match cap.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ if enabled => res += mul_cap(cap),
            _ => {}
        }
    }
    println!("{res}");
}

util::aoc_main!();
