use std::fs;
use std::process::exit;
use std::str::FromStr;
use std::cell::RefCell;

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

    // Return a so that a <op> b = result
    fn find0(&self, b: i64, result: i64) -> i64 {
        match self {
            Self::Add => result - b,
            Self::Sub => result + b,
            Self::Mul => result / b,
            Self::Div => result * b,
        }
    }

    // Return b so that a <op> b = result
    fn find1(&self, a: i64, result: i64) -> i64 {
        match self {
            Self::Add => result - a,
            Self::Sub => a - result,
            Self::Mul => result / a,
            Self::Div => a / result,
        }
    }
}


enum Monkey<'a> {
    Number(i64),
    Operation {
        monkey0: &'a str,
        needs_humn0: RefCell<bool>,
        monkey1: &'a str,
        needs_humn1: RefCell<bool>,
        operation: Operation,
    },
}

impl Monkey<'_> {
    fn get_value(&self, monkeys: &FxHashMap<&str, Monkey>) -> i64 {
        match self {
            Monkey::Number(n) => *n,
            Monkey::Operation {
                monkey0,
                monkey1,
                operation,
                ..
            } => {
                operation.apply(monkeys[monkey0].get_value(monkeys),
                                monkeys[monkey1].get_value(monkeys))
            },
        }
    }

    // Recurse until reaching humn to find the wanted value
    fn should_be(&self, monkeys: &FxHashMap<&str, Monkey>, result: i64) -> i64 {
        if let Monkey::Operation {
            monkey0,
            needs_humn0,
            monkey1,
            needs_humn1,
            operation,
        } = self {
            if *needs_humn0.borrow() {
                let val = operation.find0(monkeys[monkey1].get_value(monkeys), result);
                if monkey0 == &"humn" {
                    val
                } else {
                    monkeys[monkey0].should_be(monkeys, val)
                }
            } else if *needs_humn1.borrow() {
                let val = operation.find1(monkeys[monkey0].get_value(monkeys), result);
                if monkey1 == &"humn" {
                    val
                } else {
                    monkeys[monkey1].should_be(monkeys, val)
                }
            } else {
                panic!("humn not found")
            }
        } else {
            panic!("Encountered leaf")
        }
    }

    // Assumes self is root. Starts the process by requiring the humn subtree to equal the result
    // of the other subtree. Also assumes that humn is more than 1 level deep.
    fn start_at_root(&self, monkeys: &FxHashMap<&str, Monkey>) -> i64 {
        if let Monkey::Operation {
            monkey0,
            needs_humn0,
            monkey1,
            needs_humn1,
            ..
        } = self {
            if *needs_humn0.borrow() {
                monkeys[monkey0].should_be(monkeys, monkeys[monkey1].get_value(monkeys))
            } else if *needs_humn1.borrow() {
                monkeys[monkey1].should_be(monkeys, monkeys[monkey0].get_value(monkeys))
            } else {
                panic!("humn not found")
            }
        } else {
            panic!("Number at root")
        }
    }

    // Mark the branch that depends on humn
    fn mark_humn(&self, monkeys: &FxHashMap<&str, Monkey>) -> bool {
        match self {
            Monkey::Number(_) => false,
            Monkey::Operation {
                monkey0,
                needs_humn0,
                monkey1,
                needs_humn1,
                ..
            } => {
                if monkey0 == &"humn" || monkeys[monkey0].mark_humn(monkeys) {
                    needs_humn0.replace(true);
                    true
                } else if monkey1 == &"humn" || monkeys[monkey1].mark_humn(monkeys) {
                    needs_humn1.replace(true);
                    true
                } else {
                    false
                }
            }
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
    monkeys["root"].mark_humn(&monkeys);
    println!("{}", monkeys["root"].start_at_root(&monkeys));
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
                needs_humn0: RefCell::new(false),
                monkey1,
                needs_humn1: RefCell::new(false),
                operation,
            });
        }
    }
    Some(table)
}
