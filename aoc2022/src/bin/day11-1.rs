use std::fs;
use std::process::exit;

use regex::Regex;
use lazy_static::lazy_static;

static INPUT: &str = "input/day11.txt";
const N_ROUNDS: u32 = 10000;

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {}", e);
            exit(1);
        });
    let mut monkeys = parse_file(&input).unwrap_or_else(|| {
        eprintln!("Malformed file");
        exit(2);
    });
    let reduction: u128 = monkeys.iter()
        .map(|m| m.as_ref().unwrap().test_div as u128)
        .product();
    for _ in 0..N_ROUNDS {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].take().unwrap().turn(&mut monkeys, reduction);
            monkeys[i].replace(monkey);
        }
    }
    
    // Use bitwise-NOT for reverse sort
    monkeys.sort_by_key(|m| !m.as_ref().unwrap().score);
    //println!("{:#?}", monkeys);
    println!("{}", monkeys[0].as_ref().unwrap().score * monkeys[1].as_ref().unwrap().score);
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: char,
    value: u32,
    test_div: u32,
    monkey_true: usize,
    monkey_false: usize,
    score: u64,
}

impl Monkey {
    fn inspect(&self, item: u128) -> u128 {
        let n = if self.value == u32::MAX {
            item
        } else {
            self.value.into()
        };
        match self.operation {
            '+' => item + n,
            '*' => item * n,
            _ => 0,
        }
    }

    fn next_monkey(&self, worry: u128) -> usize {
        if worry % self.test_div as u128 == 0 {
            self.monkey_true
        } else {
            self.monkey_false
        }
    }

    fn turn(mut self, monkeys: &mut Vec<Option<Monkey>>, reduction: u128) -> Self{
        for item in &self.items {
            let new_worry = self.inspect(*item) % reduction;
            let next = self.next_monkey(new_worry);
            monkeys[next].as_mut().unwrap().items.push(new_worry);
            self.score += 1;
        }
        self.items.clear();
        self
    }
}

fn parse_file(input: &str) -> Option<Vec<Option<Monkey>>> {
    lazy_static! {
        static ref PAT_OP: Regex = Regex::new(r"Operation: new = old ([+*]) (\d+|old)").unwrap();
    }
    let mut monkeys = Vec::new();
    let lines = &mut input.lines();
    loop {
        // "Monkey N:" line
        lines.next()?;
        // Items
        let items = lines.next()?
            .get(18..)?
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();
        // Operation
        let caps = PAT_OP.captures(lines.next()?).unwrap();
        let operation = caps.get(1)?.as_str().chars().next()?;
        let value = match caps.get(2)?.as_str() {
            "old" => u32::MAX,
            n => n.parse().ok()?,
        };
        // Test
        let test_div = lines.next()?.get(21..)?.parse().ok()?;
        let monkey_true = lines.next()?.get(29..)?.parse().ok()?;
        let monkey_false = lines.next()?.get(30..)?.parse().ok()?;

        monkeys.push(Some(Monkey {
            items,
            operation,
            value,
            test_div,
            monkey_true,
            monkey_false,
            score: 0,
        }));

        match lines.next() {
            Some(_) => (),
            None => break,
        }
    }

    return Some(monkeys);
}
