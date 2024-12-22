fn step(n: u32) -> u32 {
    let a = (n ^ (n << 6)) % (1 << 24);
    let b = a ^ (a >> 5);
    (b ^ (b << 11)) % (1 << 24)
}

fn part1(input: String) {
    let sum = input
        .lines()
        .map(|l| {
            let n = l.parse().unwrap();
            (0..2000).fold(n, |acc, _| step(acc)) as u64
        })
        // More than 2¹⁰ 24-bit numbers requires 35 bits
        .sum::<u64>();
    println!("{sum}");
}

const N_SEQUENCES: usize = 19usize.pow(4);

fn sequence_key(sequence: &[i8]) -> usize {
    sequence
        .iter()
        .enumerate()
        .map(|(i, x)| (x + 9) as usize * 19usize.pow(i as u32))
        .sum()
}

fn part2(input: String) {
    // Table for collecting the amount of bananas for every possible sequence
    let mut table = vec![0; N_SEQUENCES];
    // Mark the sequences we encountered in a round to ensure that only the first occurence is used
    let mut seen = vec![false; N_SEQUENCES];
    for l in input.lines() {
        let n = l.parse().unwrap();
        let (diffs, prices): (Vec<i8>, Vec<u8>) = (0..2000)
            .scan(n, |acc, _| {
                let next = step(*acc);
                let diff = (next % 10) as i8 - (*acc % 10) as i8;
                *acc = next;
                Some((diff, (next % 10) as u8))
            })
            .unzip();
        for (window, price) in diffs.windows(4).zip(prices.iter().skip(3)) {
            let key = sequence_key(window);
            if !seen[key] {
                seen[key] = true;
                table[key] += *price as u32;
            }
        }
        // Reset seen sequences for next round
        seen.fill(false);
    }
    let bananas = table.iter().max().unwrap();
    println!("{bananas}");
}

util::aoc_main!();
