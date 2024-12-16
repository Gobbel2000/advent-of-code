use std::cmp::{Ordering, Reverse};

use euclid::{default::*, vec2};
use priority_queue::PriorityQueue;
use rustc_hash::{FxHashMap, FxHashSet};

const DIRS: [Vector2D<i32>; 4] = [vec2(1, 0), vec2(0, 1), vec2(-1, 0), vec2(0, -1)];

type Node = (Point2D<i32>, u8);

fn parse(input: &str) -> (Vec<Vec<bool>>, Point2D<i32>, Point2D<i32>) {
    let mut start = None;
    let mut end = None;
    let mut field = Vec::new();
    for (y, l) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, b) in l.bytes().enumerate() {
            if b == b'S' {
                start = Some(Point2D::new(x, y).to_i32());
            } else if b == b'E' {
                end = Some(Point2D::new(x, y).to_i32());
            }
            row.push(b == b'#');
        }
        field.push(row);
    }
    (field, start.unwrap(), end.unwrap())
}

fn adj(field: &[Vec<bool>], (v, dir): Node) -> Vec<(Node, u32)> {
    let mut adj = Vec::with_capacity(3);
    let next = v + DIRS[dir as usize];
    if !field[next.y as usize][next.x as usize] {
        adj.push(((next, dir), 1));
    }
    adj.push(((v, (dir + 1) % 4), 1000));
    adj.push(((v, (dir + 3) % 4), 1000));
    adj
}

fn shortest_path_length(field: &[Vec<bool>], start: Node, end: Point2D<i32>) -> u32 {
    let mut dist: FxHashMap<Node, u32> = FxHashMap::default();
    dist.insert(start, 0);
    let mut pq: PriorityQueue<Node, Reverse<u32>> = PriorityQueue::new();
    pq.push(start, Reverse(0));
    while let Some((v, _)) = pq.pop() {
        for (w, weight) in adj(field, v) {
            let dist_w = dist.get(&w).copied().unwrap_or(u32::MAX);
            let new_dist = dist[&v] + weight;
            if dist_w > new_dist {
                dist.insert(w, new_dist);
                pq.push_increase(w, Reverse(new_dist));
            }
        }
    }
    // Shortest distance to end, regardless of final direction
    (0..4).map(|dir| dist[&(end, dir)]).min().unwrap()
}

fn part1(input: String) {
    let (field, start, end) = parse(&input);
    let distance = shortest_path_length(&field, (start, 0), end);
    println!("{distance}");
}

fn shortest_path_tiles(field: &[Vec<bool>], start: Node, end: Point2D<i32>) -> u32 {
    let mut parents: FxHashMap<Node, Vec<Node>> = FxHashMap::default();
    let mut dist: FxHashMap<Node, u32> = FxHashMap::default();
    dist.insert(start, 0);
    let mut pq: PriorityQueue<Node, Reverse<u32>> = PriorityQueue::new();
    pq.push(start, Reverse(0));
    while let Some((v, _)) = pq.pop() {
        for (w, weight) in adj(field, v) {
            let dist_w = dist.get(&w).copied().unwrap_or(u32::MAX);
            let new_dist = dist[&v] + weight;
            match dist_w.cmp(&new_dist) {
                Ordering::Greater => {
                    parents.insert(w, vec![v]);
                    dist.insert(w, new_dist);
                    pq.push_increase(w, Reverse(new_dist));
                }
                // Remember both parents if distance is equal
                Ordering::Equal => parents.get_mut(&w).unwrap().push(v),
                Ordering::Less => {}
            }
        }
    }
    let mut path_tiles: FxHashSet<Point2D<i32>> = FxHashSet::default();
    path_tiles.insert(end);

    // Shortest distance to end, regardless of final direction
    let shortest_dist = (0..4).map(|dir| dist[&(end, dir)]).min().unwrap();
    for dir in 0..4 {
        if dist[&(end, dir)] == shortest_dist {
            collect_tiles(&parents, &mut path_tiles, (end, dir));
        }
    }
    path_tiles.len() as u32
}

fn collect_tiles(
    parents: &FxHashMap<Node, Vec<Node>>,
    tiles: &mut FxHashSet<Point2D<i32>>,
    cur: Node,
) {
    if let Some(pars) = parents.get(&cur) {
        for p in pars {
            tiles.insert(p.0);
            collect_tiles(parents, tiles, *p);
        }
    }
}

fn part2(input: String) {
    let (field, start, end) = parse(&input);
    let tiles = shortest_path_tiles(&field, (start, 0), end);
    println!("{tiles}");
}

util::aoc_main!();
