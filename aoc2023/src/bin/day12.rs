use std::str::FromStr;
use std::collections::HashMap;

use rayon::prelude::*;

#[derive(Debug)]
struct SpringRow {
    springs: Vec<Option<bool>>,
    ranges: Vec<usize>,
}

impl SpringRow {
    // Naive, uncached looped version, still faster in part 1, takes forever in part 2
    fn arrangements(&self) -> u64 {
        // Stack of branch positions, used for backtracking
        let mut decisions = Vec::new();
        // Final count
        let mut n_arrangements = 0;
        // Index into springs
        let mut si = 0;
        // Index into ranges
        let mut ri = 0;
        // Determines what to do when encountering '?'. If not backtracking, assume '.',
        // and then try '#' while backtracking.
        let mut backtracked = false;

        loop {
            let mut backtrack = false;
            match self.springs[si] {
                Some(false) => si += 1,
                None if !backtracked => {
                    decisions.push((si, ri));
                    si += 1;
                },
                Some(true) | None => {
                    let range = self.ranges[ri];
                    if self.can_advance(si, range) {
                        si += range + 1;
                        ri += 1;
                        if ri == self.ranges.len() {
                            // All ranges accounted for, no '#' may follow in the end
                            if si >= self.springs.len() ||
                                self.springs[si..].iter().all(|s| *s != Some(true))
                            {
                                n_arrangements += 1;
                            }
                            backtrack = true;
                        }
                    } else {
                        backtrack = true;
                    }
                },
            }
            if si >= self.springs.len() {
                backtrack = true;
            }

            backtracked = backtrack;
            if backtrack {
                match decisions.pop() {
                    None => {
                        // Exhausted all options, return
                        return n_arrangements;
                    },
                    Some((a, b)) => { si = a; ri = b },
                }
            }
        }
    }

    // Recursive with dynamic programming (intermediate results are cached)
    fn recursive(&self) -> u64 {
        let mut cache: HashMap<(usize, usize), u64> = HashMap::new();
        self.recurse(0, 0, &mut cache)
    }

    fn recurse(&self, si: usize, ri: usize, cache: &mut HashMap<(usize, usize), u64>) -> u64 {
        // Exhausted ranges
        if ri == self.ranges.len() {
            // All ranges accounted for, no '#' may follow in the end
            if si >= self.springs.len() ||
                self.springs[si..].iter().all(|s| *s != Some(true))
            {
                return 1;
            } else {
                // Invalid result, there are still '#' that follow
                return 0;
            }
        }

        // Exhausted springs, this path was invalid
        if si >= self.springs.len() {
            return 0;
        }

        if let Some(res) = cache.get(&(si, ri)) {
            return *res;
        }

        // We are at '.', move ahead by one
        let spring = self.springs[si];
        if let Some(false) = spring {
            return self.recurse(si + 1, ri, cache);
        }

        // We are now at '#' or '?'
        let range = self.ranges[ri];
        // Number of arrangements when assuming we are at a '#'
        let mut arrangements = if self.can_advance(si, range) {
            self.recurse(si + range + 1, ri + 1, cache) 
        } else { 0 };

        // We are at a '?', consider skipping
        if spring.is_none() {
            arrangements += self.recurse(si + 1, ri, cache);
        }

        cache.insert((si, ri), arrangements);
        arrangements
    }

    fn can_advance(&self, si: usize, range: usize) -> bool {
        let end = si + range;
        if end > self.springs.len() {
            return false;
        }
        for i in si..end {
            if let Some(false) = self.springs[i] {
                return false;
            }
        }
        // Next field is '#', the continuous range is too long
        if let Some(Some(true)) = self.springs.get(end) {
            return false;
        }
        true
    }

    fn unfold(&self, n: usize) -> Self {
        let spring_repeat = vec![self.springs.clone(); n];
        let springs = spring_repeat.join(&None);
        let ranges = self.ranges.repeat(n);

        Self {
            springs,
            ranges,
        }
    }
}

impl FromStr for SpringRow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs_s, ranges_s) = s.split_once(' ').unwrap();
        let springs = springs_s.bytes().map(|b| match b {
            b'.' => Some(false),
            b'#' => Some(true),
            b'?' => None,
            _ => panic!("Invalid character"),
        }).collect();
        let ranges = ranges_s.split(',').map(|n| n.parse().unwrap()).collect();
        Ok(SpringRow {
            springs,
            ranges,
        })
    }
}

fn part1(input: String) {
    let rows: Vec<SpringRow> = input.lines().map(|l| l.parse().unwrap()).collect();    
    let sum: u64 = rows.iter().map(|r| r.arrangements()).sum();
    println!("{sum}");
}

fn part2(input: String) {
    let rows: Vec<SpringRow> = input.lines()
        .map(|l| l.parse::<SpringRow>().unwrap().unfold(5))
        .collect();    
    let sum: u64 = rows.par_iter().map(|r| r.recursive()).sum();
    println!("{sum}");
}

util::aoc_main!("day12.txt");
