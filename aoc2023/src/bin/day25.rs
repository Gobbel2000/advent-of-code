use std::collections::{HashMap, VecDeque};

struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Look if the 3 edges actually separate the graph, if so return the 2 sizes.
    fn sizes(&self, edges: [(usize, usize); 3]) -> Option<(u32, u32)> {
        let emain = edges[0];
        let mut size = 0;
        let mut frontier = VecDeque::from([emain.0]);
        let mut visited = vec![false; self.adj.len()];
        while let Some(node) = frontier.pop_front() {
            // There is actually a connection between the two halves.
            if node == emain.1 {
                return None
            }
            for &next in &self.adj[node] {
                if !edges.contains(&(node, next)) &&
                    !edges.contains(&(next, node)) &&
                    !visited[next]
                {
                    visited[next] = true;
                    frontier.push_back(next);
                    size += 1;
                }
            }
        }
        let other_size = self.adj.len() as u32 - size;
        Some((size, other_size))
    }

    // Run BFS from every starting point to see if the frontier becomes 3 wide
    fn search(&self, start: usize) -> Option<[(usize, usize); 3]> {
        // Store edges in frontier. The start point does not have a predecessor.
        let mut frontier = VecDeque::from([(usize::MAX, start)]);
        let mut visited = vec![false; self.adj.len()];
        while let Some((_, node)) = frontier.pop_front() {
            for &next in &self.adj[node] {
                if !visited[next] {
                    visited[next] = true;
                    frontier.push_back((node, next));
                }
            }
            if frontier.len() == 3 {
                return Some([frontier[0], frontier[1], frontier[2]]);
            }
        }
        None
    }
}

fn get_graph(input: String) -> Graph {
    let mut adj = Vec::new();
    let mut names: HashMap<&str, usize> = HashMap::new();
    for line in input.lines() {
        let (start, adjs) = line.split_once(": ").unwrap();
        let idx = new_node(start, &mut adj, &mut names);
        for next in adjs.split_whitespace() {
            let nidx = new_node(next, &mut adj, &mut names);
            // Graph is undirected
            adj[idx].push(nidx);
            adj[nidx].push(idx);
        }
    }
    Graph { adj }
}

fn new_node<'a>(
    name: &'a str,
    adj: &mut Vec<Vec<usize>>,
    names: &mut HashMap<&'a str, usize>
) -> usize {
    if let Some(i) = names.get(name) {
        *i
    } else {
        let i = adj.len();
        adj.push(Vec::new());
        names.insert(name, i);
        i
    }
}

fn part1(input: String) {
    let graph = get_graph(input);
    for start in 0..graph.adj.len() {
        if let Some(sep) = graph.search(start) {
            if let Some((a, b)) = graph.sizes(sep) {
                println!("{a}, {b}");
                println!("{}", a * b);
                break;
            }
        }
    }
}

fn part2(_i: String) {
    println!("No part 2 for day 25");
}

util::aoc_main!("day25.txt");
