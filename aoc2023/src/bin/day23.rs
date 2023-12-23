use std::collections::VecDeque;
use std::cmp::Ordering;

use ndarray::Array2;
use rustc_hash::FxHashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    N,
    E,
    S,
    W,
}

use Dir::*;

impl From<u8> for Dir {
    fn from(c: u8) -> Self {
        match c {
            b'^' => N,
            b'>' => E,
            b'v' => S,
            b'<' => W,
            _ => panic!("Invalid direction char: {c}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Spot {
    Path,
    Forest,
    Slope(Dir),
}

fn parse_input(input: String) -> Array2<Spot> {
    let lines: Vec<&str> = input.lines().collect();
    let elements: Vec<Spot> = lines.iter().flat_map(|l|
            l.bytes().map(|b| match b {
                b'.' => Spot::Path,
                b'#' => Spot::Forest,
                slope => Spot::Slope(slope.into()),
            })
        )
        .collect();
    Array2::from_shape_vec((lines.len(), elements.len() / lines.len()), elements).unwrap()
}

fn end_points(field: &Array2<Spot>) -> ((usize, usize), (usize, usize)) {
    let start = (0, 1);
    assert!(matches!(field[start], Spot::Path));
    let end = (field.nrows() - 1, field.ncols() - 2);
    assert!(matches!(field[end], Spot::Path));
    (start, end)
}

fn get_adj(field: &Array2<Spot>, pos: (usize, usize), prev_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let slope = if let Spot::Slope(dir) = field[pos] {
        Some(dir)
    } else {
        None
    };
    let mut adj = Vec::with_capacity(4);
    if pos.0 > 0 && !slope.is_some_and(|dir| dir != W) {
        adj.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 && !slope.is_some_and(|dir| dir != N) {
        adj.push((pos.0, pos.1 - 1));
    }
    if pos.0 + 1 < field.nrows() && !slope.is_some_and(|dir| dir != S) {
        adj.push((pos.0 + 1, pos.1));
    }
    if pos.1 + 1 < field.ncols() && !slope.is_some_and(|dir| dir != E) {
        adj.push((pos.0, pos.1 + 1));
    }
    adj.retain(|&p| p != prev_pos && field[p] != Spot::Forest);
    adj
}

fn get_adj_no_slopes(field: &Array2<Spot>, pos: (usize, usize), prev_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adj = Vec::with_capacity(4);
    if pos.0 > 0 {
        adj.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        adj.push((pos.0, pos.1 - 1));
    }
    if pos.0 + 1 < field.nrows() {
        adj.push((pos.0 + 1, pos.1));
    }
    if pos.1 + 1 < field.ncols() {
        adj.push((pos.0, pos.1 + 1));
    }
    adj.retain(|&p| p != prev_pos && field[p] != Spot::Forest);
    adj
}

struct SearchPos {
    pos: (usize, usize),
    // For not going backwards
    prev_pos: (usize, usize),
    prev_node: usize,
    dist: u32,
}

fn make_graph(field: &Array2<Spot>) -> (Vec<Vec<(usize, u32)>>, usize) {
    let (start, end) = end_points(field);
    let mut graph: Vec<Vec<(usize, u32)>> = vec![vec![]];
    let mut nodes = FxHashMap::default();

    let mut queue = VecDeque::from([SearchPos {
        pos: start,
        dist: 0,
        prev_node: 0,
        prev_pos: (usize::MAX, usize::MAX),
    }]);
    let mut end_node = None;
    while let Some(searchpos) = queue.pop_front() {
        let pos = searchpos.pos;
        if pos == end {
            if let Some(&node) = nodes.get(&pos) {
                graph[searchpos.prev_node].push((node, searchpos.dist));
                graph[node].push((searchpos.prev_node, searchpos.dist));
            } else {
                let new_node = graph.len();
                graph[searchpos.prev_node].push((new_node, searchpos.dist));
                graph.push(vec![(searchpos.prev_node, searchpos.dist)]);
                nodes.insert(pos, new_node);
                end_node = Some(new_node);
            }
            continue;
        }

        let adj = get_adj_no_slopes(field, searchpos.pos, searchpos.prev_pos);
        match adj.len().cmp(&1) {
            Ordering::Equal => {
                queue.push_back(SearchPos {
                    pos: adj[0], 
                    prev_pos: searchpos.pos,
                    dist: searchpos.dist + 1,
                    prev_node: searchpos.prev_node,
                })
            },
            Ordering::Greater => {
                // Found a junction
                if let Some(&node) = nodes.get(&pos) {
                    if !graph[searchpos.prev_node].contains(&(node, searchpos.dist)) {
                        // Do not duplicate edges, if it is discovered from both ends
                        graph[searchpos.prev_node].push((node, searchpos.dist));
                        graph[node].push((searchpos.prev_node, searchpos.dist));
                    }
                    continue;
                }
                let new_node = graph.len();
                graph[searchpos.prev_node].push((new_node, searchpos.dist));
                nodes.insert(pos, new_node);
                graph.push(vec![(searchpos.prev_node, searchpos.dist)]);
                for next in adj {
                    queue.push_back(SearchPos {
                        pos: next,
                        prev_pos: searchpos.pos,
                        prev_node: new_node,
                        dist: 1,
                    });
                }
            },
            Ordering::Less => {},
        }
    }
    (graph, end_node.unwrap())
}

fn graph_longest_path(graph: &[Vec<(usize, u32)>], start: usize, end: usize) -> u32 {
    let mut max_dist = 0;
    let mut seen_start = vec![false; graph.len()];
    seen_start[start] = true;
    let mut queue = VecDeque::from([(start, 0, seen_start)]);
    while let Some((node, dist, mut seen)) = queue.pop_front() {
        if node == end {
            max_dist = max_dist.max(dist);
            continue;
        }
        let n_adj = graph[node].len();
        for (next, weight) in &graph[node][..n_adj - 1] {
            if !seen[*next] {
                let mut path = seen.clone();
                path[node] = true;
                queue.push_back((*next, dist + weight, path));
            }
        }
        // Do not clone seen array for the last edge
        if n_adj > 0 {
            let (next, weight) = &graph[node][n_adj - 1];
            if !seen[*next] {
                seen[node] = true;
                queue.push_back((*next, dist + weight, seen));
            }

        }
    }
    max_dist
}

fn longest_path(field: &Array2<Spot>) -> u32 {
    let (start, end) = end_points(field);
    let mut queue = VecDeque::from([(start, 0, (usize::MAX, usize::MAX))]);
    let mut max_end_dist = 0;
    while let Some((pos, dist, prev_pos)) = queue.pop_front() {
        if pos == end {
            max_end_dist = max_end_dist.max(dist);
        }
        for adj in get_adj(field, pos, prev_pos) {
            queue.push_back((adj, dist + 1, pos));
        }
    }
    max_end_dist as u32
}

fn part1(input: String) {
    let field = parse_input(input);
    println!("{}", longest_path(&field));
}

fn part2(input: String) {
    let field = parse_input(input);
    let (graph, end) = make_graph(&field);
    let length = graph_longest_path(&graph, 0, end);
    println!("{}", length);
}

util::aoc_main!("day23.txt");
