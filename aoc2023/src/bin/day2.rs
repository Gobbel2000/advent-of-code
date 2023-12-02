use std::cmp::Ordering;
use std::str::FromStr;
use std::sync::OnceLock;

use regex::Regex;


#[derive(Debug, Clone, Default, PartialEq)]
struct CubeSet {
    red: u32,
    blue: u32,
    green: u32,
}

impl CubeSet {
    fn max(&self, other: &Self) -> Self {
        CubeSet {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for CubeSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = {
            static ONCE: OnceLock<Regex> = OnceLock::new();
            ONCE.get_or_init(|| Regex::new(r"\s*(?<n>\d+) (?<color>(?:red)|(?:green)|(?:blue))").unwrap())
        };
        let mut new = CubeSet::default();
        for cap in re.captures_iter(s) {
            let n = cap.name("n").unwrap().as_str().parse().unwrap();
            match cap.name("color").unwrap().as_str() {
                "red" => new.red = n,
                "green" => new.green = n,
                "blue" => new.blue = n,
                _ => unreachable!(),
            }
        }
        Ok(new)
    }
}

impl PartialOrd for CubeSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            return Some(Ordering::Less);
        }
        if self.red >= other.red && self.green >= other.green && self.blue >= other.blue {
            return Some(Ordering::Greater);
        }
        // Incomparable
        None
    }
}

// Return the id of the game and an iterator of cube sets for a given line
fn iter_sets<'a>(line: &'a str) -> (u32, impl Iterator<Item=CubeSet> + 'a) {
    let re_game = {
        static ONCE: OnceLock<Regex> = OnceLock::new();
        ONCE.get_or_init(|| Regex::new(r"Game (?<n>\d+):\s*").unwrap())
    };
    let cap = re_game.captures(line).expect("Line does not start with 'Game N: '");
    let id = cap.name("n").unwrap().as_str().parse().unwrap();
    let sets_str = &line[cap.get(0).unwrap().end()..];
    (id, sets_str.split(';').map(|s| CubeSet::from_str(s).unwrap()))
}

fn part1(input: String) {
    const MAX_CUBES: CubeSet = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut sum = 0;
    for line in input.lines() {
        let (id, mut sets) = iter_sets(line);
        if sets.all(|set| set <= MAX_CUBES) {
            sum += id;
        }
    }
    println!("{sum}");
}

fn part2(input: String) {
    let mut sum = 0;
    for line in input.lines() {
        let (_id, sets) = iter_sets(line);
        let max = sets.reduce(|acc, set| acc.max(&set)).expect("There should be at least one round");
        sum += max.power(); 
    }
    println!("{sum}");
}

util::aoc_main!("day2.txt");
