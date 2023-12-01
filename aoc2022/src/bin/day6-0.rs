use std::fs;
use std::process::exit;

static INPUT: &str = "input/day6.txt";
const MARK_LEN: usize = 4;

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {}", e);
            exit(1);
        });
    let bytes = input.as_bytes();
    for i in MARK_LEN..bytes.len() {
        if check_marker(&bytes[i-MARK_LEN..i]) {
            println!("{}", i);
            exit(0);
        }
    }
    println!("No marker found!");
}

fn check_marker(marker: &[u8]) -> bool {
    let len = marker.len();
    for i in 0..len {
        for j in i+1..len {
            if marker[i] == marker[j] {
                return false;
            }
        }
    }
    return true;
}
