use std::cmp::Ordering;
use std::ops::Range;
use std::collections::HashMap;
use std::sync::OnceLock;

use regex::Regex;

// Part 2 tells us that all numbers are bounded within 1..=4000
type Part = [u16; 4];
type PartRanges = [Range<u16>; 4];

struct Rule {
    condition: Option<Condition>,
    next: RuleType,
}

struct Condition {
    idx: usize,
    cmp: Ordering,
    num: u16,
}

impl Condition {
    fn test(&self, part: Part) -> bool {
        part[self.idx].cmp(&self.num) == self.cmp
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum RuleType {
    Workflow(usize),
    Accept,
    Reject,
}

impl RuleType {
    fn new(s: &str, names: &HashMap<&str, usize>) -> Self {
        match s {
            "A" => RuleType::Accept,
            "R" => RuleType::Reject,
            name => RuleType::Workflow(names[name]),
        }
    }
}

fn parse_rules(input: &str) -> (Vec<Vec<Rule>>, usize) {
    let lines: Vec<_> = input.lines().collect();
    let mut names = HashMap::with_capacity(lines.len());
    let mut rule_strings = Vec::with_capacity(lines.len());
    for (i, l) in lines.iter().enumerate() {
        let (name, rule) = l.split_once('{').unwrap();
        names.insert(name, i);
        rule_strings.push(rule.strip_suffix('}').unwrap());
    }
    let mut rules = Vec::with_capacity(lines.len());
    for r in rule_strings {
        rules.push(r.split(',').map(|rule| {
            match rule.split_once(':') {
                None => Rule { condition: None, next: RuleType::new(rule, &names) },
                Some((cond_s, next)) => {
                    let cmp = if let Some(">") = cond_s.get(1..2) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    };
                    let idx = match cond_s.get(0..1) {
                        Some("x") => 0,
                        Some("m") => 1,
                        Some("a") => 2,
                        Some("s") => 3,
                        _ => panic!("Invalid variable"),
                    };
                    let num = cond_s.get(2..).unwrap().parse().unwrap();
                    Rule {
                        condition: Some(Condition { idx, cmp, num }),
                        next: RuleType::new(next, &names)
                    }
                },
            }
        })
        .collect());
    }
    (rules, names["in"])
}

fn parse_part(line: &str) -> Part {
    let re = {
        static ONCE: OnceLock<Regex> = OnceLock::new();
        ONCE.get_or_init(|| Regex::new(r"(\d+).*?(\d+).*?(\d+).*?(\d+)").unwrap())
    };
    let cap = re.captures(line).unwrap();
    let mut part = [0; 4];
    for (i, m) in cap.iter().skip(1).enumerate() {
        part[i] = m.unwrap().as_str().parse().unwrap();
    }
    part
}

fn parse_input(input: String) -> (Vec<Vec<Rule>>, usize, Vec<Part>) {
    let (rule_s, part_s) = input.split_once("\n\n").unwrap();
    let (rules, input) = parse_rules(rule_s);
    let parts = part_s.lines().map(parse_part).collect();
    (rules, input, parts)
}


fn run_workflow(rules: &[Rule], part: Part) -> RuleType {
    for r in rules {
        if r.condition.as_ref().map(|c| c.test(part)).unwrap_or(true) {
            return r.next;
        }
    }
    unreachable!("One rule must always apply")
}

fn run_rules(rules: &[Vec<Rule>], input: usize, part: Part) -> bool {
    let mut rule_idx = input;
    loop {
        match run_workflow(&rules[rule_idx], part) {
            RuleType::Accept => return true,
            RuleType::Reject => return false,
            RuleType::Workflow(next) => rule_idx = next,
        }
    }
}

fn part1(input: String) {
    let (rules, input, parts) = parse_input(input);
    let sum: u32 = parts.iter()
        .filter(|p| run_rules(&rules, input, **p))
        .flat_map(|p| p.iter().map(|e| *e as u32))
        .sum();
    println!("{sum}");
}


fn accepted_ranges(rules: Vec<Vec<Rule>>, input: usize) -> Vec<PartRanges> {
    recurse(&rules, input, vec![[1..4001, 1..4001, 1..4001, 1..4001]])
}

// Return all ranges within start_ranges that would be accepted when starting at the rule specified by `idx`.
fn recurse(rules: &[Vec<Rule>], idx: usize, start_ranges: Vec<PartRanges>) -> Vec<PartRanges> {
    let mut out = Vec::new();
    // Ranges that apply for the next rule
    let mut cur_ranges = start_ranges.clone();
    for rule in &rules[idx] {
        // Ranges that would fall into the current rule
        let mut branch_ranges = Vec::new();
        if let Some(cond) = &rule.condition {
            let i = cond.idx;
            for r in cur_ranges.iter_mut() {
                // Mutate cur_ranges inplace so it doesn't fall into the current rule
                // Build branch_ranges so the current rule applies
                let mut r_branch = r.clone();
                if cond.cmp == Ordering::Greater {
                    r_branch[i].start = r_branch[i].start.max(cond.num + 1);
                    r[i].end = r[i].end.min(cond.num + 1);
                } else { // Less
                    r[i].start = r[i].start.max(cond.num);
                    r_branch[i].end = r[i].end.min(cond.num);
                }
                // Filter out empty ranges in branch_ranges
                if !r_branch[i].is_empty() {
                    branch_ranges.push(r_branch);
                }
            }
            // Filter out empty ranges in cur_ranges
            cur_ranges.retain(|r| !r[cond.idx].is_empty());
        } else {
            branch_ranges = cur_ranges.clone();
        }
        match rule.next {
            RuleType::Workflow(next) => out.extend(recurse(rules, next, branch_ranges)),
            RuleType::Accept => out.extend(branch_ranges),
            RuleType::Reject => {},
        }
    }
    out
}

fn part2(input: String) {
    let (rule_s, _parts) = input.split_once("\n\n").unwrap();
    let (rules, input) = parse_rules(rule_s);
    let final_ranges = accepted_ranges(rules, input);
    let sum: u64 = final_ranges.iter()
        .map(|ranges|
            ranges.iter().map(|r| (r.end - r.start) as u64).product::<u64>()
        )
        .sum();
    println!("{sum}");

}

util::aoc_main!("day19.txt");
