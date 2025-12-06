fn part1(input: String) {
    let mut nums: Vec<Vec<u64>> = Vec::new();
    let mut mul: Vec<bool> = Vec::new();
    for l in input.lines() {
        if l.chars().next().unwrap().is_ascii_digit() {
            let row = l
                .split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            nums.push(row);
        } else {
            mul = l.split_ascii_whitespace().map(|s| s == "*").collect();
        }
    }
    let mut sum = 0;
    for (idx, op_mul) in mul.iter().enumerate() {
        let col = nums.iter().map(|row| row[idx]);
        sum += if *op_mul {
            col.reduce(|acc, n| acc * n)
        } else {
            col.reduce(|acc, n| acc + n)
        }
        .unwrap();
    }
    println!("{sum}");
}

fn part2(input: String) {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let n_rows = grid.len() - 1; // Not counting operator row
    let mut op_mul = grid[n_rows][0] == b'*';
    let mut cur = if op_mul { 1 } else { 0 };
    let mut sum = 0;
    for x in 0..grid[0].len() {
        let digits: Vec<u8> = (0..n_rows).map(|y| grid[y][x]).collect();
        if digits.iter().all(|d| *d == b' ') {
            sum += cur;
            op_mul = grid[n_rows][x + 1] == b'*';
            cur = if op_mul { 1 } else { 0 };
            continue;
        }
        let n = String::from_utf8(digits)
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap();
        if op_mul {
            cur *= n;
        } else {
            cur += n;
        }
    }
    sum += cur;
    println!("{sum}");
}

util::aoc_main!();
