fn flatten_block(block: Vec<Vec<bool>>) -> [u8; 5] {
    let mut flat = [0; 5];
    for row in &block[1..=5] {
        for x in 0..5 {
            if row[x] {
                flat[x] += 1;
            }
        }
    }
    flat
}

fn parse(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for block_s in input.split("\n\n") {
        let block: Vec<Vec<bool>> = block_s
            .lines()
            .map(|l| l.bytes().map(|b| b == b'#').collect::<Vec<bool>>())
            .collect();
        assert_eq!(block.len(), 7);
        // Lock
        if block[0].iter().all(|e| *e) {
            locks.push(flatten_block(block));
        } else {
            keys.push(flatten_block(block));
        }
    }
    (locks, keys)
}

fn part1(input: String) {
    let (locks, keys) = parse(&input);
    let mut count = 0u32;
    for l in locks {
        for k in &keys {
            if l.iter().zip(k).map(|(li, ki)| li + ki).all(|sum| sum <= 5) {
                count += 1;
            }
        }
    }
    println!("{count}");
}

fn part2(_input: String) {
    println!("⭐");
}

util::aoc_main!();
