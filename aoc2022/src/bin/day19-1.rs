use std::fs;
use std::process::exit;
use std::str::FromStr;
use std::cmp::Ordering;

use derive_more::{Add, AddAssign, Sub, SubAssign};
use fxhash::FxHashMap;
use lazy_static::lazy_static;
use regex::Regex;

static INPUT: &str = "input/day19.txt";
const ROUNDS: u32 = 32;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Default, Add, AddAssign, Sub, SubAssign)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ords = vec![
            self.ore.cmp(&other.ore),
            self.clay.cmp(&other.clay),
            self.obsidian.cmp(&other.obsidian),
            self.geode.cmp(&other.geode),
        ];
        if ords.iter().all(|o| o.is_eq()) {
            Some(Ordering::Equal)
        } else if ords.iter().all(|o| o.is_le()) {
            Some(Ordering::Less)
        } else if ords.iter().all(|o| o.is_ge()) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

struct Blueprint {
    ore: Resources,
    clay: Resources,
    obsidian: Resources,
    geode: Resources,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseBlueprintError;

impl FromStr for Blueprint {
    type Err = ParseBlueprintError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PAT: Regex = Regex::new(&(r"Blueprint \d+: ".to_owned() +
                r"Each ore robot costs (\d+) ore. " +
                r"Each clay robot costs (\d+) ore. " +
                r"Each obsidian robot costs (\d+) ore and (\d+) clay. " +
                r"Each geode robot costs (\d+) ore and (\d+) obsidian.")
            ).unwrap();
        }
        let empty = Resources::default();
        let caps = PAT.captures(s).ok_or(ParseBlueprintError)?;
        let ore = Resources {
            ore: caps.get(1).unwrap().as_str().parse().map_err(|_| ParseBlueprintError)?,
            ..empty
        };
        let clay = Resources {
            ore: caps.get(2).unwrap().as_str().parse().map_err(|_| ParseBlueprintError)?,
            ..empty
        };
        let obsidian = Resources {
            ore: caps.get(3).unwrap().as_str().parse().map_err(|_| ParseBlueprintError)?,
            clay: caps.get(4).unwrap().as_str().parse().map_err(|_| ParseBlueprintError)?,
            ..empty
        };
        let geode = Resources {
            ore: caps.get(5).unwrap().as_str().parse().map_err(|_| ParseBlueprintError)?,
            obsidian: caps.get(6).unwrap().as_str().parse().map_err(|_| ParseBlueprintError)?,
            ..empty
        };
        Ok(Blueprint {
            ore,
            clay,
            obsidian,
            geode,
        })
    }
}

struct Simulation {
    blueprint: Blueprint,
    resources: Resources,
    robots: Resources,
    max_needed: Resources,
    cache: FxHashMap<(Resources, Resources, u32), u32>,
}

impl Simulation {
    fn new(blueprint: Blueprint) -> Self {
        // Calculate the maximum resource usage per robot so we don't collect more than we can use
        // per round
        let recipes = vec![blueprint.ore, blueprint.clay, blueprint.obsidian, blueprint.geode];
        let max_needed = Resources {
            ore: recipes.iter().map(|r| r.ore).max().unwrap(),
            clay: recipes.iter().map(|r| r.clay).max().unwrap(),
            obsidian: recipes.iter().map(|r| r.obsidian).max().unwrap(),
            geode: 0,
        };
        println!("{:?}", max_needed);
        Self {
            blueprint,
            resources: Resources::default(),
            robots: Resources{ore: 1, ..Resources::default()},
            max_needed,
            cache: FxHashMap::default(),
        }
    }

    fn simulate_cached(&mut self, rounds: u32) -> u32 {
        let state = (self.resources, self.robots, rounds);
        if let Some(&val) = self.cache.get(&state) {
            return val
        }
        let val = self.simulate(rounds);
        self.cache.insert(state.clone(), val);
        val
    }

    fn simulate(&mut self, rounds: u32) -> u32 {
        if rounds == 0 {
            //println!("{:?}", self.resources);
            return self.resources.geode
        }
        let mut max_geodes = 0;
        // Schedule robot builds
        if self.blueprint.geode <= self.resources {
            self.resources -= self.blueprint.geode;
            self.resources += self.robots;
            self.robots.geode += 1;
            max_geodes = max_geodes.max(self.simulate_cached(rounds-1));
            // Backtrack
            self.resources += self.blueprint.geode;
            self.robots.geode -= 1;
            self.resources -= self.robots;
        } else {
            if self.blueprint.obsidian <= self.resources &&
            self.robots.obsidian < self.max_needed.obsidian {
                self.resources -= self.blueprint.obsidian;
                self.resources += self.robots;
                self.robots.obsidian += 1;
                max_geodes = max_geodes.max(self.simulate_cached(rounds-1));
                // Backtrack
                self.resources += self.blueprint.obsidian;
                self.robots.obsidian -= 1;
                self.resources -= self.robots;
            }
            if self.blueprint.clay <= self.resources &&
            self.robots.clay < self.max_needed.clay {
                self.resources -= self.blueprint.clay;
                self.resources += self.robots;
                self.robots.clay += 1;
                max_geodes = max_geodes.max(self.simulate_cached(rounds-1));
                // Backtrack
                self.resources += self.blueprint.clay;
                self.robots.clay -= 1;
                self.resources -= self.robots;
            }
            if self.blueprint.ore <= self.resources &&
            self.robots.ore < self.max_needed.ore {
                self.resources -= self.blueprint.ore;
                self.resources += self.robots;
                self.robots.ore += 1;
                max_geodes = max_geodes.max(self.simulate_cached(rounds-1));
                // Backtrack
                self.resources += self.blueprint.ore;
                self.robots.ore -= 1;
                self.resources -= self.robots;
            }
            // Try skipping
            self.resources += self.robots;
            max_geodes = max_geodes.max(self.simulate_cached(rounds-1));
            self.resources -= self.robots;
        }
        max_geodes
    }
}

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not open input file: {e}");
            exit(1);
        });
    let blueprints: Vec<Blueprint> = input.lines().enumerate()
        // Only take the first 3 blueprints
        .filter(|(i, _)| *i < 3)
        .map(|(_, l)| l.parse().unwrap_or_else(|_| {
            eprintln!("Error while parsing");
            exit(2);
        })).collect();
    let score: u32 = blueprints.into_iter()
        .map(|b| {
            let mut sim = Simulation::new(b);
            let geodes = sim.simulate(ROUNDS);
            println!("{geodes}");
            geodes
        }).product();
    println!("{score}");
}
