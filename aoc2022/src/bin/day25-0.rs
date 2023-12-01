use std::fs;
use std::process::exit;

static INPUT: &str = "input/day25.txt";
const BASE: i64 = 5;

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not open input file: {e}");
            exit(1);
        });
    let total_fuel = input.lines()
        .map(snafu_to_int)
        .sum();
    //println!("{}", total_fuel);
    let snafu_fuel = int_to_snafu(total_fuel);
    assert_eq!(total_fuel, snafu_to_int(&snafu_fuel));
    println!("{}", snafu_fuel);
}

fn snafu_to_int(snafu: &str) -> i64 {
    let mut n = 0;
    let mut radix: i64 = 1;
    for b in snafu.as_bytes().iter().rev() {
        match b {
            b'0'..=b'2'  => n += (b - b'0') as i64 * radix,
            b'-' => n -= radix,
            b'=' => n -= 2 * radix,
            _ => panic!("Invalid character: {b}"),
        }
        radix *= BASE;
    }
    n
}

fn int_to_snafu(mut n: i64) -> String {
    if n < 0 {
        return "-".to_string() + &int_to_snafu(-n);
    } else if n == 0 {
        return "0".to_string();
    }
    let mut snafu: Vec<u8> = Vec::new();
    while n > 0 {
        let rem = n % BASE;
        n /= BASE;
        n += match rem {
            0..=2 => { snafu.push(rem as u8 + b'0'); 0 },
            3 => { snafu.push(b'='); 1 },
            4 => { snafu.push(b'-'); 1 },
            _ => unreachable!("rem can only be between 0 and 4"),
        };
    }

    snafu.reverse();
    String::from_utf8(snafu).expect("Should be valid UTF-8 characters")
}
