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
    let score = best_score(forest);
    println!("{}", score); 
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

fn best_score(forest: Vec<Vec<u8>>) -> u32 {
    let n = forest.len();
    let m = forest[0].len();
    let mut max = 0;
    for i in 0..n {
        for j in 0..m {
            let score = scenic_score(&forest, (i, j));
            if score > max {
                max = score;
            }
        }
    }
    max
}

fn scenic_score(forest: &Vec<Vec<u8>>, tree: (usize, usize)) -> u32 {
    let n = forest.len();
    let m = forest[0].len();
    let height = forest[tree.0][tree.1];

    let mut up = 0;
    for i in (0..tree.0).rev() {
        up += 1;
        if forest[i][tree.1] >= height {
            break;
        }
    }

    let mut right = 0;
    for i in tree.1+1..m {
        right += 1;
        if forest[tree.0][i] >= height {
            break;
        }
    }

    let mut down = 0;
    for i in tree.0+1..n {
        down += 1;
        if forest[i][tree.1] >= height {
            break;
        }
    }

    let mut left = 0;
    for i in (0..tree.1).rev() {
        left += 1;
        if forest[tree.0][i] >= height {
            break;
        }
    }

    up * right * down * left
}
