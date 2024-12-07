fn parse_line(l: &str) -> (u64, Vec<u64>) {
    let (first, last) = l.split_once(": ").unwrap();
    let solution = first.parse().unwrap();
    let values = last.split_whitespace().map(|s| s.parse().unwrap()).collect();
    (solution, values)
}

fn sum_valid(input: &str, valid: fn(u64, u64, &[u64]) -> bool) -> u64 {
    let mut sum = 0;
    for l in input.lines() {
        let (solution, values) = parse_line(l);
        if valid(solution, values[0], &values[1..]) {
            sum += solution;
        }
    }
    sum
}

fn part1(input: String) {
    fn is_valid(solution: u64, acc: u64, values: &[u64]) -> bool {
        match values.first() {
            None => acc == solution,
            Some(n) => is_valid(solution, acc + n, &values[1..]) ||
                is_valid(solution, acc * n, &values[1..]),
        }
    }

    println!("{}", sum_valid(&input, is_valid));
}

fn part2(input: String) {
    fn is_valid(solution: u64, acc: u64, values: &[u64]) -> bool {
        match values.first() {
            None => acc == solution,
            Some(n) => is_valid(solution, acc + n, &values[1..]) ||
                is_valid(solution, acc * n, &values[1..]) ||
                is_valid(solution, acc * 10u64.pow(n.ilog10() + 1) + n, &values[1..]), 
        }
    }

    println!("{}", sum_valid(&input, is_valid));
}

util::aoc_main!();
