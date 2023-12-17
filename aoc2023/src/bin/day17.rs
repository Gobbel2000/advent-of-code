use std::cmp::Ordering;
use std::ops::RangeInclusive;
use std::collections::BinaryHeap;

use ndarray::Array2;

fn parse_input(input: String) -> Array2<u8> {
    let lines: Vec<_> = input.lines().collect();
    let n_lines = lines.len();
    let elements: Vec<_> = lines.iter().flat_map(|l|
            l.chars().map(|c| c.to_digit(10).unwrap() as u8)
        )
        .collect();
    Array2::from_shape_vec((n_lines, elements.len() / n_lines), elements)
        .unwrap()
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: Position,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip ordering to get min-heap instead of max-heap
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Position {
    position: (usize, usize),
    horizontal: bool,
}

impl Position {
    fn adj(&self, field: &Array2<u8>, moves: RangeInclusive<usize>) -> Vec<(Self, u32)> {
        let (pos, weights) = if self.horizontal {
            (self.position.0, field.column(self.position.1))
        } else {
            (self.position.1, field.row(self.position.0))
        };
        let mut positions = Vec::new();
        let mut weight: u32 = 0;
        for i in 1..=*moves.end() {
            if pos < i {
                break;
            }
            let pos2 = pos - i;
            weight += weights[pos2] as u32;
            if i >= *moves.start() {
                let new_pos = if self.horizontal {
                    (pos2, self.position.1)
                } else {
                    (self.position.0, pos2)
                };
                positions.push((Position { position: new_pos, horizontal: !self.horizontal }, weight));
            }
        }
        weight = 0;
        for i in 1..=*moves.end() {
            let pos2 = pos + i;
            if pos2 >= weights.len() {
                break;
            }
            weight += weights[pos2] as u32;
            if i >= *moves.start() {
                let new_pos = if self.horizontal {
                    (pos2, self.position.1)
                } else {
                    (self.position.0, pos2)
                };
                positions.push((Position { position: new_pos, horizontal: !self.horizontal }, weight));
            }
        }
        positions
    }

    // Index into dist array
    fn key(&self, field: &Array2<u8>) -> usize {
        (self.position.0 * field.ncols() + self.position.1) * 2
            + self.horizontal as usize
    }

    // Manhattan distance as heuristic for A*
    fn heuristic(&self, end: (usize, usize)) -> u32 {
        ((end.0 - self.position.0) + (end.1 - self.position.1)) as u32
    }
}

fn astar(field: Array2<u8>, moves: RangeInclusive<usize>) -> Option<u32> {
    let start = (0, 0);
    let end = (field.nrows() - 1, field.ncols() - 1,);
    
    let mut dist = vec![u32::MAX; field.len() * 2];
    // Try starting in both directions
    let starts = [
        Position { position: start, horizontal: false },
        Position { position: start, horizontal: true },
    ];
    dist[starts[0].key(&field)] = 0;
    dist[starts[1].key(&field)] = 0;
    let mut heap = BinaryHeap::from([
        State { cost: starts[0].heuristic(end), position: starts[0] },
        State { cost: starts[1].heuristic(end), position: starts[1] },
    ]);

    while let Some(State { cost, position }) = heap.pop() {
        if position.position == end {
            return Some(cost)
        }
        for (next, weight) in position.adj(&field, moves.clone()) {
            let new_dist = dist[position.key(&field)] + weight;
            if new_dist < dist[next.key(&field)] {
                dist[next.key(&field)] = new_dist;
                heap.push(State {
                    cost: new_dist + next.heuristic(end),
                    position: next,
                });
            }
        }
    }
    // End unreachable
    None
}

fn part1(input: String) {
    let field = parse_input(input);    
    println!("{}", astar(field, 1..=3).unwrap());
}

fn part2(input: String) {
    let field = parse_input(input);    
    println!("{}", astar(field, 4..=10).unwrap());
}

util::aoc_main!("day17.txt");
