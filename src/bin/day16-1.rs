use std::fs;
use std::process::exit;
use std::collections::{HashMap, VecDeque};

use regex::Regex;
use lazy_static::lazy_static;

static INPUT: &str = "input/day16.txt";
const ROUNDS: u32 = 26;

#[derive(Clone, Debug)]
struct Node {
    rate: u32,
    adj: Vec<usize>
}

#[derive(Debug)]
struct Solver {
    graph: WeightedGraph,
    steps: Vec<Vec<usize>>,
    time: Vec<u32>,
    pos: Vec<usize>,
}

impl Solver {
    fn new(graph: WeightedGraph, n_players: usize) -> Self {
        Self {
            graph,
            steps: vec![vec![0]; n_players],
            time: vec![ROUNDS; n_players],
            pos: vec![0; n_players],
        }
    }

    fn score(&self) -> u32 {
        let mut score = 0;
        for steps in self.steps.iter() {
            let mut t = ROUNDS;
            for i in 1..steps.len() {
                t -= self.graph.weights[steps[i-1]][steps[i]];
                score += self.graph.rates[steps[i]] * t;
            }
        }
        score
    }

    fn advance(&mut self, next: usize, player: usize) {
        self.time[player] -= self.graph.weights[*self.steps[player].last().unwrap()][next];
        self.steps[player].push(next);
        self.pos[player] = next;
    }

    fn backtrack(&mut self, player: usize) {
        let last = self.steps[player].pop().unwrap();
        let prev = *self.steps[player].last().unwrap();
        self.time[player] += self.graph.weights[prev][last];
        self.pos[player] = prev;
    }

    fn options(&self, player: usize, time_left: u32) -> Vec<usize> {
        (1..self.graph.len()).into_iter()
            .filter(|idx| {
                for s in self.steps.iter().flatten() {
                    if idx == s {
                        return false;
                    }
                }
                self.graph.weights[self.pos[player]][*idx] <= time_left
            }).collect()
    }

    fn search(mut self) -> (Self, u32) {
        let mut max_score = 0;
        let opt0 = self.options(0, self.time[0]);
        if opt0.is_empty() {
            let opt1 = self.options(1, self.time[1]);
            return self.search_moves(&opt1, 1);
        }

        for m0 in opt0 {
            self.advance(m0, 0);
            let opt1 = self.options(1, self.time[1]);
            let score;
            (self, score) = self.search_moves(&opt1, 1);
            if score > max_score {
                max_score = score;
            }
            self.backtrack(0);
        }
        (self, max_score)
    }

    fn search_moves(mut self, moves: &[usize], player: usize) -> (Self, u32) {
        if moves.is_empty() {
            let score = self.score();
            return (self, score);
        }
        let mut max_score = 0;
        for &m in moves {
            self.advance(m, player);
            let score;
            (self, score) = self.search();
            if score > max_score {
                max_score = score;
            }
            self.backtrack(player);
        }
        (self, max_score)
    }
}

#[derive(Debug)]
struct WeightedGraph {
    rates: Vec<u32>,
    weights: Vec<Vec<u32>>,
}

impl WeightedGraph {
    fn new(rates: &[u32]) -> Self {
        Self {
            rates: Vec::from(rates),
            weights: vec![vec![0; rates.len()]; rates.len()],
        }
    }

    fn add_edge(&mut self, v: usize, w: usize, weight: u32) {
        self.weights[v][w] = weight;
        self.weights[w][v] = weight;
    }

    fn len(&self) -> usize {
        self.rates.len()
    }
}

fn get_weighted_graph(nodes: &[Node], start: usize) -> WeightedGraph {
    let mut rates = vec![nodes[start].rate];
    let mut valuable: Vec<usize> = vec![start];
    let mut to_val_idx: Vec<Option<usize>> = vec![None; nodes.len()];
    to_val_idx[start] = Some(0);
    for (i, n) in nodes.iter().enumerate() {
        if n.rate > 0 && i != start {
            to_val_idx[i] = Some(rates.len());
            rates.push(n.rate);
            valuable.push(i);
        }
    }
    let mut graph = WeightedGraph::new(&rates);

    // Calculate weights (distances in original graph)
    for v in valuable {
        let mut dist_to_v: Vec<u32> = vec![0; nodes.len()];
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(v);
        let mut visited: Vec<bool> = vec![false; nodes.len()];
        visited[v] = true;
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            let dist = dist_to_v[u];
            for &w in nodes[u].adj.iter() {
                if !visited[w] {
                    visited[w] = true;
                    dist_to_v[w] = dist + 1;
                    if nodes[w].rate > 0 {
                        graph.add_edge(to_val_idx[v].unwrap(), to_val_idx[w].unwrap(),
                           // Add one more to account for time opening the valve
                           dist + 2);
                    }
                    q.push_back(w);
                }
            }
        }
    }
    graph
}

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {e}");
            exit(1);
        });
    let (all_nodes, start) = parse_input(&input);
    let graph = get_weighted_graph(&all_nodes, start);
    //println!("{:?}", graph);
    let solver = Solver::new(graph, 2);
    let (_solver, score) = solver.search();
    println!("{score}");
}

fn parse_input(input: &str) -> (Vec<Node>, usize) {
    lazy_static! {
        static ref PAT: Regex = Regex::new(
            r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ")
            .unwrap();
    }
    let mut nodes = Vec::new();
    let mut code_to_idx: HashMap<&str, usize> = HashMap::new();
    let mut adjacent_code: Vec<Vec<&str>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let caps = PAT.captures(line).unwrap_or_else(|| {
            eprintln!("Malformed input: {}", line);
            exit(4);
        });
        let code = caps.get(1).unwrap().as_str();
        code_to_idx.insert(code, i);
        let rate = caps.get(2).unwrap().as_str().parse().unwrap();
        let match_size = caps.get(0).unwrap().end();
        let adj = line[match_size..].split(", ")
            .collect();
        adjacent_code.push(adj);
        nodes.push(Node {
            rate,
            adj: Vec::new(),
        });
    }
    for (i, adj_code) in adjacent_code.iter().enumerate() {
        let adj: Vec<usize> = adj_code.iter()
            .map(|code| *code_to_idx.get(code).unwrap())
            .collect();
        nodes[i].adj = adj;
    }
    (nodes, code_to_idx["AA"])
}
