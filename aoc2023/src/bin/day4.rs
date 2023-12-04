use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

struct Scratchcard {
    winning: HashSet<u32>,
    drawn: HashSet<u32>,
}

impl Scratchcard {
    // Amount of winning numbers we have drawn
    fn n_winning(&self) -> usize {
        (&self.winning & &self.drawn).len()
    }

    fn points(&self) -> u32 {
        match self.n_winning() {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

impl FromStr for Scratchcard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_card, numbers) = s.split_once(':').ok_or("Missing ':'")?;
        let (s_winning, s_drawn) = numbers.split_once('|').ok_or("Missing '|'")?;
        let winning = s_winning.split_whitespace()
            .map(u32::from_str)
            .collect::<Result<HashSet<u32>, ParseIntError>>()
            .map_err(|e| e.to_string())?;
        let drawn = s_drawn.split_whitespace()
            .map(u32::from_str)
            .collect::<Result<HashSet<u32>, ParseIntError>>()
            .map_err(|e| e.to_string())?;

        Ok(Scratchcard {
            winning,
            drawn,
        })
    }
}

fn part1(input: String) {
    let sum: u32 = input.lines()
        .map(|l| l.parse::<Scratchcard>().unwrap().points())
        .sum();
    println!("{sum}");
}

fn part2(input: String) {
    let scratchcards: Vec<Scratchcard> = input.lines()
        .map(|l| l.parse().unwrap())
        .collect();
    // How many copies we have of each card. We start out with one of each kind.
    let mut amounts = vec![1; scratchcards.len()];
    for (i, card) in scratchcards.iter().enumerate() {
        let copies = amounts[i];
        for amount in amounts.iter_mut().skip(i + 1).take(card.n_winning()) {
            *amount += copies;
        }
    }
    println!("{}", amounts.iter().sum::<u32>());
}

util::aoc_main!("day4.txt");
