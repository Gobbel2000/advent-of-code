use std::fs;
use std::process::exit;

static INPUT: &str = "input/day10.txt";

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {}", e);
            exit(1);
        });
    
    let x_per_cycle = register_history(&input);
    let mut i = 20;
    let mut total = 0;
    loop {
        if i > x_per_cycle.len() {
            break;
        }
        total += x_per_cycle[i] * (i as i32); 
        i += 40;
    }

    println!("{}", total);
}

fn register_history(input: &str) -> Vec<i32> {
    let mut x_per_cycle: Vec<i32> = Vec::new();
    let mut x = 1;
    // Pad for 1-indexed list
    x_per_cycle.push(0);
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
