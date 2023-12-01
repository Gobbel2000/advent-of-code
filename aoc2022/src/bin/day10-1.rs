use std::fs;
use std::process::exit;

static INPUT: &str = "input/day10.txt";

const SCREEN_WIDTH: usize = 40;
const SCREEN_HEIGHT: usize = 6;

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {}", e);
            exit(1);
        });
    let x_per_cycle = register_history(&input);
    println!("{}", simulate_crt(x_per_cycle));
}

fn register_history(input: &str) -> Vec<i32> {
    let mut x_per_cycle: Vec<i32> = Vec::new();
    let mut x = 1;
    for line in input.lines() {
        let instr = Instruction::from_str(line).unwrap_or_else(|| {
            eprintln!("Malformed input");
            exit(2);
        });

        x_per_cycle.push(x);
        match instr {
            Instruction::Addx(val) => {
                // One more cycle
                x_per_cycle.push(x);
                x += val;
            },
            Instruction::Noop => continue,
        }
    }
    return x_per_cycle;
}

fn simulate_crt(register: Vec<i32>) -> String {
    let mut screen = [['.'; SCREEN_WIDTH]; SCREEN_HEIGHT];
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let i = y*40 + x;
            // Set pixel to "on" if it is close to the sprite location in the register
            if (x as i32).abs_diff(register[i]) <= 1 {
                screen[y][x] = '#';
            }
        }
    }
    // Convert each line into a string
    let lines: Vec<String> = screen.iter()
        .map(|row| {
            let mut s = String::with_capacity(SCREEN_WIDTH + 1);
            for i in 0..SCREEN_WIDTH {
                s.push(row[i]);
            }
            s.push('\n');
            s
        }).collect();
    // Join all lines together
    let mut lines_it = lines.into_iter();
    let first = lines_it.next().unwrap();
    return lines_it.fold(first, |acc, l| acc + &l);
}

enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn from_str(string: &str) -> Option<Self> {
        if string == "noop" {
            Some(Self::Noop)
        } else if string.starts_with("addx ") {
            let val = string.get(5..)?.parse().ok()?;
            Some(Self::Addx(val))
        } else {
            None
        }
    }
}
