use std::collections::VecDeque;

use num::Integer;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
struct Graph {
    modules: Vec<Module>,
    adj: Vec<Vec<usize>>,
    useful: Vec<Option<usize>>,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(bool),
    Conjunction(FxHashMap<usize, bool>),
    Untyped(Option<bool>),
}

impl Module {
    fn process(&mut self, signal: bool) -> Option<bool> {
        match self {
            Module::FlipFlop(ref mut state) => {
                if !signal {
                    *state = !*state;
                    Some(*state)
                } else {
                    // High pulse (true) is ignored
                    None
                }
            },
            Module::Conjunction(incoming) => {
                // Send false iff all incoming values are true
                Some(incoming.values().any(|e| !*e))
            },
            Module::Untyped(ref mut last) => {
                *last = Some(signal);
                None
            },
        }
    }
}

fn parse_input(input: String) -> (Graph, Vec<usize>) {
    let lines: Vec<_> = input.lines().collect();
    let mut modules = Vec::with_capacity(lines.len() - 1);
    let mut adj = Vec::with_capacity(lines.len() - 1);
    let mut start = Vec::new();
    
    let mut names = FxHashMap::default();
    let mut offset = 0;
    for (i, l) in lines.iter().enumerate() {
        let (name, _) = l.split_once(" -> ").unwrap();
        if name != "broadcaster" {
            names.insert(&name[1..], i - offset);
        } else {
            offset = 1;
        }
    }

    let mut untyped = Vec::new();
    for l in lines {
        let (module, adj_s) = l.split_once(" -> ").unwrap();
        let module_adj = adj_s.split(", ").map(|name|
            names.get(name).copied().unwrap_or_else(|| {
                untyped.push(Module::Untyped(None));
                names.len() + untyped.len() - 1
            })
        ).collect();
        if module == "broadcaster" {
            start = module_adj;
        } else {
            match &module[..1] {
                "%" => modules.push(Module::FlipFlop(false)),
                "&" => modules.push(Module::Conjunction(FxHashMap::default())),
                _ => panic!("Invalid module type prefix in {}", module),
            }
            adj.push(module_adj);
        }
    }

    for _ in 0..untyped.len() {
        adj.push(Vec::new());
    }
    modules.extend(untyped);

    let rev: Vec<Vec<usize>> = (0..adj.len()).map(|i|
        adj.iter().enumerate()
            .filter_map(|(j, row)| row.contains(&i).then_some(j))
            .collect()
        )
        .collect();

    for i in 0..modules.len() {
        // Find incoming signals for each conjunction module
        if let Module::Conjunction(ref mut incoming) = modules[i] {
            for j in &rev[i] {
                incoming.insert(*j, false);
            }
        }
    }

    // Find the conjunctions that feed into rx. This is based on quite a few assumptions about the
    // input.
    let mut useful = vec![None; modules.len()];
    let rev_rx = rev.last().unwrap();
    // RX should have exactly one predecessor
    assert_eq!(rev_rx.len(), 1);
    let rx_source = rev_rx[0];
    // That predecessor should be a conjunction module
    assert!(matches!(modules[rx_source], Module::Conjunction(_)));
    for (i, conj) in rev[rx_source].iter().enumerate() {
        // All predecessors of that conjunction module are also conjunction modules
        assert!(matches!(modules[*conj], Module::Conjunction(_)));
        useful[*conj] = Some(i);
    }

    let graph = Graph {
        modules,
        adj,
        useful,
    };

    (graph, start)
}

fn press_button_count(graph: &mut Graph, start: &[usize]) -> (u64, u64) {
    // Signal of button plus each signal of broadcast
    let mut low = 1 + start.len();
    let mut high = 0;

    let mut queue: VecDeque<(usize, bool)> = start.iter()
        .map(|&midx| (midx, false)).collect();

    while let Some((midx, signal)) = queue.pop_front() {
        if let Some(signal) = graph.modules[midx].process(signal) {
            let n_adj = graph.adj[midx].len();
            if signal {
                high += n_adj;
            } else {
                low += n_adj;
            }
            for i in 0..n_adj {
                let next = graph.adj[midx][i];
                // Update incoming memory of conjunction modules
                if let Module::Conjunction(ref mut incoming) = graph.modules[next] {
                    incoming.insert(midx, signal);
                }
                queue.push_back((next, signal));
            }
        }
    }

    (low as u64, high as u64)
}

fn press_button(graph: &mut Graph, start: &[usize], cycles: &mut [Option<u64>], iteration: u64) {
    let mut queue: VecDeque<(usize, bool)> = start.iter()
        .map(|&midx| (midx, false)).collect();

    while let Some((midx, signal)) = queue.pop_front() {
        if let Some(signal) = graph.modules[midx].process(signal) {
            let n_adj = graph.adj[midx].len();
            for i in 0..n_adj {
                let next = graph.adj[midx][i];
                // Update incoming memory of conjunction modules
                if let Module::Conjunction(ref mut incoming) = graph.modules[next] {
                    // If it is one of the critical conjunctions that feeds into rx, remember the
                    // iteration and assume it just cycles from there.
                    if let Some(idx) = graph.useful[next] {
                        if !signal && cycles[idx].is_none() {
                            cycles[idx] = Some(iteration);
                        }
                    }
                    incoming.insert(midx, signal);
                }
                queue.push_back((next, signal));
            }
        }
    }
}

fn part1(input: String) {
    let (mut modules, start) = parse_input(input);     
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let (l, h) = press_button_count(&mut modules, &start);
        low += l;
        high += h;
    }

    println!("Low signals: {}, High signals: {}", low, high);
    println!("Solution: {}", low * high);
}

fn part2(input: String) {
    let (mut graph, start) = parse_input(input);     
    let mut cycles = vec![None; graph.useful.iter().filter(|e| e.is_some()).count()];
    for i in 1u64.. {
        press_button(&mut graph, &start, &mut cycles, i);
        if let Module::Untyped(Some(false)) =  graph.modules.last().unwrap() {
            println!("{i}");
            break;
        }
        if cycles.iter().all(|c| c.is_some()) {
            println!("Cycles of subgraphs: {:?}", cycles);
            let lcm = cycles.iter()
                .map(|e| e.unwrap())
                .reduce(|acc, e| acc.lcm(&e))
                .unwrap();
            println!("LCM: {lcm}");
            break;
        }
    }
}

util::aoc_main!("day20.txt");
