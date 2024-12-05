use std::collections::{HashSet, HashMap, VecDeque};

fn parse_lists(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|l| l.split(',').map(|e| e.parse().unwrap()).collect())
        .collect()
}

fn parse_relation(input: String) -> (HashSet<(u32, u32)>, Vec<Vec<u32>>) {
    let (ordering, lists) = input.split_once("\n\n").unwrap();
    let relation = ordering.lines()
        .map(|l| {
            let (a, b) = l.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    (relation, parse_lists(lists))
}

fn parse_graph(input: String) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let (ordering, lists) = input.split_once("\n\n").unwrap();
    let mut graph = Vec::new();
    for l in ordering.lines() {
        let (a, b) = l.split_once('|').unwrap();
        let v: u32 = a.parse().unwrap();
        let w: u32 = b.parse().unwrap();
        let new_len = v.max(w) as usize + 1;
        if new_len > graph.len() {
            graph.resize(new_len, Vec::new())
        }
        graph[v as usize].push(w);
    }
    (graph, parse_lists(lists))
}


fn part1(input: String) {
    let (relation, lists) = parse_relation(input); 
    let mut sum = 0;
    for l in lists {
        let mut valid = true;
        for i in 0..l.len() {
            for j in 0..i {
                if relation.contains(&(l[i], l[j])) {
                    valid = false;
                    break
                }
            }
            if !valid { break }
        }
        if valid {
            sum += l[l.len() / 2];
        }
    }
    println!("{sum}");
}


// Topological order of graph, but limited to nodes in the set `subgraph`.
// Otherwise the graph is not acyclic.
fn topological_sort(graph: &[Vec<u32>], subgraph: &HashSet<u32>) -> Vec<u32> {
    let mut order = VecDeque::with_capacity(subgraph.len());
    let mut marked = vec![false; graph.len()];
    for &v in subgraph {
        if !marked[v as usize] {
            dfs(graph, subgraph, v as usize, &mut marked, &mut order)
        }
    }
    order.into()
}

fn dfs(graph: &[Vec<u32>], subgraph: &HashSet<u32>, v: usize, marked: &mut [bool], order: &mut VecDeque<u32>) {
    marked[v] = true;
    for &w in graph[v].iter().filter(|v| subgraph.contains(v)) {
        if !marked[w as usize] {
            dfs(graph, subgraph, w as usize, marked, order);
        }
    }
    order.push_front(v as u32);
}

fn rank(order: &[u32]) -> HashMap<u32, u32> {
    order.iter().enumerate().map(|(i, x)| (*x, i as u32)).collect()
}

// Part 1 with topological sorting, which is slower
fn _part1(input: String) {
    let (graph, lists) = parse_graph(input);
    let mut sum = 0;
    for l in lists {
        let subgraph = HashSet::from_iter(l.iter().copied());
        let rank = rank(&topological_sort(&graph, &subgraph));
        if l.is_sorted_by_key(|x| rank[x]) {
            sum += l[l.len() / 2];
        }
    }
    println!("{sum}");
}

fn part2(input: String) {
    let (graph, lists) = parse_graph(input);
    let mut sum = 0;
    for mut l in lists {
        let subgraph = HashSet::from_iter(l.iter().copied());
        let rank = rank(&topological_sort(&graph, &subgraph));
        if !l.is_sorted_by_key(|x| rank[x]) {
            l.sort_unstable_by_key(|x| rank[x]);            
            sum += l[l.len() / 2];
        }
    }
    println!("{sum}");
}

util::aoc_main!();
