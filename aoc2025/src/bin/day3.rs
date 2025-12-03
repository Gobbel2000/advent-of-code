fn part1(input: String) {
    let mut sum = 0;
    'banks: for l in input.lines() {
        let mut sorted: Vec<(usize, u32)> = l
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .collect();
        sorted.sort_by(|(_, a), (_, b)| a.cmp(b).reverse());
        for (idx, first) in &sorted {
            for (id2, second) in &sorted {
                if id2 > idx {
                    sum += first * 10 + second;
                    continue 'banks;
                }
            }
        }
    }
    println!("{sum}");
}

// Recursive implementation of greedy algorithm.
// Returns Vec of length 12 if a result was found, guaranteed to be optimal.
// If there is no solution with the input, a shorter Vec is returned.
fn recursive(bank: &[(usize, u32)], mut cur: Vec<(usize, u32)>) -> Vec<(usize, u32)> {
    let pos = cur.last().unwrap().0;
    for &(idx, e) in bank.iter().filter(|(idx, _)| *idx > pos) {
        cur.push((idx, e));
        if cur.len() == 12 {
            // Recursion anchor: We have filled all 12 spots and therefore found
            // the best solution
            return cur;
        }
        // Recurse
        cur = recursive(bank, cur);
        if cur.len() == 12 {
            // Result found
            return cur;
        }
        // Nothing found, try next in this position
        cur.pop();
    }
    // Unsuccessful search with given inputs
    cur
}

fn part2(input: String) {
    let mut sum = 0;
    'banks: for l in input.lines() {
        let mut sorted: Vec<(usize, u32)> = l
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .enumerate()
            .collect();
        sorted.sort_by(|(_, a), (_, b)| a.cmp(b).reverse());
        let mut cur: Vec<(usize, u32)> = Vec::with_capacity(12);
        for &(idx, first) in &sorted {
            cur.push((idx, first));
            cur = recursive(&sorted, cur);
            if cur.len() == 12 {
                let num = cur.iter().fold(0u64, |acc, e| acc * 10 + e.1 as u64);
                sum += num;
                continue 'banks;
            }
            cur.pop();
        }
    }
    println!("{sum}");
}

util::aoc_main!();
