use std::fs;
use std::process::exit;

use regex::Regex;
use lazy_static::lazy_static;

static INPUT: &str = "input/day5.txt";

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {}", e);
            exit(1);
        });
    let cut = input.find("\n\n").unwrap();
    let stack_s = &input[..cut];
    let moves_s = &input[cut+2..];

    let mut stacks = read_stacks(stack_s);

    for line in moves_s.lines() {
        let next_move = read_move(line).unwrap_or_else(|| {
            eprintln!("Malformed input moves");
            exit(2);
        });

        apply_move(next_move, &mut stacks);
    }

    for stack in stacks.iter() {
        print!("[{}] ", stack.last().unwrap_or(&' '));
    }
    print!("\n");
}

fn read_stacks(input: &str) -> Box<Vec<Vec<char>>> {
    let lines: Vec<&str> = input.lines().collect();
    let n_stacks = lines[lines.len()-1]
        .split_whitespace()
        .collect::<Vec<&str>>()
        .len();
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(n_stacks);
    for _ in 0..n_stacks {
        stacks.push(Vec::new());
    }
    for line in lines.iter().rev().skip(1) {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..n_stacks {
            if let Some(c) = chars.get(i*4 + 1) {
                if c.is_ascii_alphabetic() {
                    stacks[i].push(*c);
                }
            }
        }
    }
    return Box::new(stacks);
}

fn read_move(line: &str) -> Option<(usize, usize, usize)> {
    lazy_static! {
        static ref PAT: Regex = Regex::new(r"^move\s*(\d+)\s*from\s*(\d+)\s*to\s*(\d+)")
            .unwrap();
    }
    let caps = PAT.captures(line)?;
    let amount: usize = caps.get(1)?.as_str().parse().ok()?;
    let from: usize = caps.get(2)?.as_str().parse().ok()?;
    let to: usize = caps.get(3)?.as_str().parse().ok()?;
    // Convert from 1-indexed to 0-indexed
    return Some((amount, from-1, to-1));
}

fn apply_move(next_move: (usize, usize, usize), stacks: &mut Box<Vec<Vec<char>>>) {
    let (amount, from, to) = next_move;
    for _ in 0..amount {
        let element = stacks[from].pop().unwrap();
        stacks[to].push(element);
    }
}
