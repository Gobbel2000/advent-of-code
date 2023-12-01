/*
 * Another implementation for Day 12 Part 2
 * but faster. This does not use A* and iterates over every starting point at height 0, but instead
 * creates a Shortest Single Source Path Tree from the End point using Dijkstra's Algorithm.
 */
use std::fs;
use std::process::exit;

use priority_queue::PriorityQueue;

static INPUT: &str = "input/day12.txt";

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {}", e);
            exit(1);
        });
    let (map, _start_pos, end_pos) = parse_map(&input);
    let graph = Graph::from_map(&map);
    let end = end_pos.0 * graph.m + end_pos.1;
    let (dist, _tree) = graph.dijkstra(end);
    let shortest = lowest_points(&map).iter()
        .map(|start_p| dist[start_p.0 * graph.m + start_p.1])
        .min().unwrap();
    // Don't count startpoint for number of steps
    println!("{}", shortest);
}

fn parse_map(input: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut map = Vec::new();
    let mut row = 0;
    let mut start = (0, 0);
    let mut end = (0, 0);
    for line in input.lines() {
        let mut col = 0;
        map.push(line.as_bytes().iter()
                 .map(|b| {
                     let height = if *b == 'S' as u8 {
                         start = (row, col);
                         0
                     } else if *b == 'E' as u8 {
                         end = (row, col);
                         25
                     } else {
                         b - 'a' as u8
                     };
                     col += 1;
                     height
                 })
                 .collect());
        row += 1;
    }
    return (map, start, end);
}

fn lowest_points(map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                points.push((i, j));
            }
        }
    }
    return points;
}

#[derive(Debug)]
struct Graph<T> {
    v: Vec<T>,
    adj: Vec<Vec<usize>>,
    _n: usize,
    m: usize,
}

impl<T> Graph<T> {
    fn dijkstra(&self, start: usize) -> (Vec<u64>, Vec<Option<usize>>) {
        let size = self.v.len();
        let mut dist = vec![u64::MAX; size];
        dist[start] = 0;
        let mut parent = vec![None; size];
        let mut pq = PriorityQueue::new();
        // PriorityQueue uses a Max-heap, so priorities are inverted using bitwise-NOT
        pq.push(start, !0);
        while !pq.is_empty() {
            let (v, _prio) = pq.pop().unwrap();
            for &w in self.adj[v].iter() {
                if dist[w] > dist[v] + 1 {
                    parent[w] = Some(v);
                    dist[w] = dist[v] + 1;
                    pq.push(w, !dist[w]);
                }
            }
        }
        return (dist, parent);

    }
}

impl Graph<u8> {
    fn from_map(map: &Vec<Vec<u8>>) -> Graph<u8> {
        let n = map.len();
        let m = map[0].len();
        let v = map.iter().flatten().copied().collect();
        let mut adj = Vec::new();
        for row in 0..n {
            for col in 0..m {
                let cur_height = map[row][col];
                let mut adjacent = Vec::new();
                for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                    let next_i = (row as isize + dy, col as isize + dx);
                    if next_i.0 >= 0 && next_i.1 >= 0 {
                        let next = (next_i.0 as usize, next_i.1 as usize);
                        if next.0 < n && next.1 < m {
                            let height = map[next.0 as usize][next.1 as usize];
                            //NOTE: Edge directions are reversed, because we search backwards
                            //starting from the endpoint
                            if height + 1 >= cur_height {
                                adjacent.push(next.0 * m + next.1);
                            }
                        }
                    }
                }
                adj.push(adjacent);
            }
        }
        Graph {
            v,
            adj,
            _n: n,
            m,
        }
    }
}
