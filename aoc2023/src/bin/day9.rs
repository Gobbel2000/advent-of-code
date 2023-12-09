fn parse_input(input: String) -> Vec<Vec<i32>> {
    input.lines()
        .map(|l| l.split_whitespace()
             .map(|n| n.parse().unwrap())
             .collect())
        .collect()
}

fn derive(ts: &[i32]) -> Vec<i32> {
    ts.windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

fn predict(input: String, last: bool) -> i32 {
    let numbers = parse_input(input); 
    let mut sum = 0;
    for ts in numbers {
        let mut derivatives: Vec<Vec<i32>> = vec![ts];
        sum += loop {
            let der = derive(derivatives.last().unwrap());
            if der.iter().all(|e| *e == 0) {
                let mut diff = 0;
                for d in derivatives.iter().rev() {
                    diff = match last {
                        true => d.last().unwrap() + diff,
                        false => d.first().unwrap() - diff,
                    };
                }
                break diff;
            }
            derivatives.push(der);
        }
    }
    sum
}

fn part1(input: String) {
    println!("{}", predict(input, true));
}

fn part2(input: String) {
    println!("{}", predict(input, false));
}

util::aoc_main!("day9.txt");
