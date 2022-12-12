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
    let (map, start_pos, end_pos) = parse_map(&input);
    let graph = Graph::from_map(map);
    //println!("{:?}", graph);
    let start = start_pos.0 * graph.m + start_pos.1;
    let end = end_pos.0 * graph.m + end_pos.1;
    let path = graph.find_path(start, end).unwrap_or_else(|| {
        eprintln!("No path found!");
        exit(9);
    });
    //println!("{:?}", path);
    // Don't count startpoint for number of steps
    println!("{}", path.len() - 1);
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

#[derive(Debug)]
struct Graph<T> {
    v: Vec<T>,
    adj: Vec<Vec<usize>>,
    _n: usize,
    m: usize,
}

impl<T> Graph<T> {
    fn find_path(&self, start: usize, end: usize) -> Option<Vec<usize>> {
        let (_dist, parent) = self.astar(start, end)?;
        let mut path = Vec::new();
        let mut cur = end;
        path.push(cur);
        while cur != start {
            cur = parent[cur].unwrap();
            path.push(cur);
        }
        path.reverse();
        return Some(path);
    }

    fn astar(&self, start: usize, end: usize) -> Option<(u64, Vec<Option<usize>>)> {
        let size = self.v.len();
        let mut pq = PriorityQueue::new();
        let mut dist = vec![u64::MAX; size];
        dist[start] = 0;
        let mut parent = vec![None; size];
        // PriorityQueue uses a Max-heap, so priorities are inverted using bitwise-NOT
        pq.push(start, !self.dist(start, end));
        while !pq.is_empty() {
            let (v, _prio) = pq.pop().unwrap();
            if v == end {
                return Some((dist[end], parent));
            }
            for &w in self.adj[v].iter() {
                if dist[w] > dist[v] + 1 {
                    parent[w] = Some(v);
                    dist[w] = dist[v] + 1;
                    // Same as above for priority
                    pq.push(w, !(dist[w] + self.dist(w, end)));
                }
            }
        }
        return None;
    }

    fn dist(&self, u: usize, v: usize) -> u64 {
        let p1 = ((u / self.m) as isize, (u % self.m) as isize);
        let p2 = ((v / self.m) as isize, (v % self.m) as isize);
        let dist = (((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2)) as f64).sqrt();
        // Must convert to integer because floats don't implement Ord
        return (dist * (1<<31) as f64).floor() as u64;
    }
}

impl Graph<u8> {
    fn from_map(map: Vec<Vec<u8>>) -> Graph<u8> {
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
                            if cur_height + 1 >= height {
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
