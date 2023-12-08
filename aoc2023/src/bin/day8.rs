use std::collections::HashMap;
use num::Integer;

trait AnyGraph {
    fn from_triples(triples: Vec<(&str, &str, &str)>) -> Self;
}

struct Graph {
    adj: Vec<[u32; 2]>,
    start: u32,
    end: u32,
}

impl AnyGraph for Graph {
    fn from_triples(triples: Vec<(&str, &str, &str)>) -> Graph {
        let names: HashMap<&str, u32> = triples.iter()
            .enumerate()
            .map(|(i, trip)| (trip.0, i as u32))
            .collect();
        let adj = triples.iter()
            .map(|trip| [names[trip.1], names[trip.2]])
            .collect();
        Graph {
            adj,
            start: names["AAA"],
            end: names["ZZZ"],
        }
    }
}

struct Graph2 {
    adj: Vec<[u32; 2]>,
    starts: Vec<u32>,
    is_end: Vec<bool>,
}

impl AnyGraph for Graph2 {
    fn from_triples(triples: Vec<(&str, &str, &str)>) -> Graph2 {
        let names: HashMap<&str, u32> = triples.iter()
            .enumerate()
            .map(|(i, trip)| (trip.0, i as u32))
            .collect();
        let mut adj = Vec::with_capacity(triples.len());
        let mut is_end = Vec::with_capacity(triples.len());
        let mut starts = Vec::new();
        for (i, trip) in triples.iter().enumerate() {
            adj.push([names[trip.1], names[trip.2]]);
            is_end.push(trip.0.ends_with('Z'));
            if trip.0.ends_with('A') {
                starts.push(i as u32);
            }
        }
        Graph2 {
            adj,
            starts,
            is_end,
        }
    }
}

fn parse_input<G: AnyGraph>(input: String) -> (Vec<u8>, G) {
    let mut lines = input.lines();
    let dir_line = lines.next().unwrap();
    let directions = dir_line.chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Invalid direction{}", c),
        })
        .collect();
    lines.next().unwrap();
    let triples = lines.map(|l| (&l[0..3], &l[7..10], &l[12..15]))
        .collect();
    let graph = G::from_triples(triples);
    (directions, graph)
}

fn part1(input: String) {
    let (directions, graph) = parse_input::<Graph>(input);
    let mut pos = graph.start;
    let mut n = 0;
    for (i, dir) in directions.iter().cycle().enumerate() {
        if pos == graph.end {
            n = i;
            break;
        }
        pos = graph.adj[pos as usize][*dir as usize];
    }
    println!("{n}");
}

fn part2(input: String) {
    let (directions, graph) = parse_input::<Graph2>(input);
    // Distance for each starting point until an end point is reached
    let mut path_lengths: Vec<u64> = Vec::with_capacity(graph.starts.len());
    for start in graph.starts {
        let mut pos = start;
        for (i, dir) in directions.iter().cycle().enumerate() {
            if graph.is_end[pos as usize] {
                path_lengths.push(i as u64);
                break;
            }
            pos = graph.adj[pos as usize][*dir as usize];
        }
    }
    println!("Path lengths: {path_lengths:?}");
    let end_all = path_lengths.into_iter()
        .reduce(|acc, n| acc.lcm(&n)) // Take least common multiple of the list
        .unwrap();
    println!("LCM: {end_all}");
}

util::aoc_main!("day8.txt");
