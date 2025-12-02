use std::ops::RangeInclusive;

fn parse_input(input: &str) -> Vec<RangeInclusive<u64>> {
    input
        .trim()
        .split(',')
        .map(|r| {
            let (a, b) = r.split_once('-').unwrap();
            RangeInclusive::new(a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn part1(input: String) {
    let ranges = parse_input(&input);
    let mut sum = 0;
    for e in ranges.into_iter().flatten() {
        let width = e.ilog10() + 1;
        if width % 2 == 0 {
            let top = 10u64.pow(width / 2);
            if e / top == e % top {
                sum += e;
            }
        }
    }
    println!("{sum}");
}

fn part2(input: String) {
    let ranges = parse_input(&input);
    let mut sum = 0;
    'nums: for e in ranges.into_iter().flatten() {
        let width = e.ilog10() + 1;
        for rep in 2..=width {
            if width % rep == 0 {
                let top = 10u64.pow(width / rep);
                let mut a = e;
                let lowest = a % top;
                let mut invalid = true;
                while a > top {
                    a /= top;
                    if a % top != lowest {
                        invalid = false;
                        break;
                    }
                }
                if invalid {
                    sum += e;
                    // Don't check other numbers of repetitions
                    continue 'nums;
                }
            }
        }
    }
    println!("{sum}");
}

util::aoc_main!();
