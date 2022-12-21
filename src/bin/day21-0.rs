use std::fs;
use std::process::exit;
use std::str::FromStr;

use fxhash::FxHashMap;

static INPUT: &str = "input/day21.txt";

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

struct ParseOpsError;

impl FromStr for Operation {
    type Err = ParseOpsError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Sub),
            "*" => Ok(Self::Mul),
            "/" => Ok(Self::Div),
            _ => Err(ParseOpsError),
        }
    }
}

impl Operation {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
        }
    }
}

enum Monkey<'a> {
    Number(i64),
    Operation {
        monkey0: &'a str,
        monkey1: &'a str,
        operation: Operation,
    },
}

impl Monkey<'_> {
    fn get_value(&self, monkeys: &FxHashMap<&str, Monkey>) -> i64 {
        match self {
            Monkey::Number(n) => *n,
            Monkey::Operation { monkey0, monkey1, operation } => {
                operation.apply(monkeys[monkey0].get_value(monkeys),
                                monkeys[monkey1].get_value(monkeys))
            },
        }
    }
}

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {e}");
            exit(1);
        });
    let monkeys = parse_input(&input).unwrap_or_else(|| {
        eprintln!("Error while parsing file");
        exit(2);
    });
    println!("{}", monkeys["root"].get_value(&monkeys));
}

fn parse_input(input: &str) -> Option<FxHashMap<&str, Monkey>> {
    let mut table = FxHashMap::default();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let name = words.next()?.trim_end_matches(':');
        let v1 = words.next()?;
        if let Ok(n) = v1.parse::<i64>() {
            table.insert(name, Monkey::Number(n));
        } else {
            let monkey0 = v1;
            let operation = words.next()?.parse().ok()?;
            let monkey1 = words.next()?;
            table.insert(name, Monkey::Operation {
                monkey0,
                monkey1,
                operation,
            });
        }
    }
    Some(table)
}
