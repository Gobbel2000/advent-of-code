use std::fs;
use std::process::exit;

static INPUT: &str = "input/day8.txt";

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {}", e);
            exit(1);
        });
    let forest = read_forest(input);
    let visible = count_visible(forest);
    println!("{}", visible); 
}

fn read_forest(input: String) -> Vec<Vec<u8>> {
    let mut forest = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap_or_else(|| {
                eprintln!("Invalid input character: {}", c);
                exit(2);
            }) as u8);
        }
        forest.push(row);
    }
    return forest;
}

fn count_visible(forest: Vec<Vec<u8>>) -> u32 {
    let n = forest.len();
    let m = forest[0].len();
    let mut visible = Vec::with_capacity(n);
    for _ in 0..n {
        let row = vec![false; m];
        visible.push(row);
    }

    // Top edge
    for i in 0..m {
        let mut max: i8 = -1;
        for j in 0..n {
            let cur = forest[j][i] as i8;
            if cur > max {
                visible[j][i] = true;
                max = cur;
            }
        }

        // Bottom edge
        let mut max: i8 = -1;
        for j in (0..n).rev() {
            let cur = forest[j][i] as i8;
            if cur > max {
                visible[j][i] = true;
                max = cur;
            }
        }
    }

    // Left edge
    for j in 0..n {
        let mut max: i8 = -1;
        for i in 0..m {
            let cur = forest[j][i] as i8;
            if cur > max {
                visible[j][i] = true;
                max = cur;
            }
        }

        // Right edge
        let mut max: i8 = -1;
        for i in (0..m).rev() {
            let cur = forest[j][i] as i8;
            if cur > max {
                visible[j][i] = true;
                max = cur;
            }
        }
    }
    
    // Count all trees marked as visible
    visible.into_iter()
        .flatten()
        .map(|b| b as u32)
        .sum()
}
