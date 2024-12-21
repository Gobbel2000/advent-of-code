use euclid::{default::*, point2, vec2};
use rustc_hash::FxHashMap;
use std::iter;

type Move = Option<Vector2D<i32>>;

const KEYPAD_GAP: Point2D<i32> = point2(0, 3);
const DPAD_GAP: Point2D<i32> = point2(0, 0);

fn keypad_pos(n: u8) -> Point2D<i32> {
    match n {
        b'7' => point2(0, 0),
        b'8' => point2(1, 0),
        b'9' => point2(2, 0),
        b'4' => point2(0, 1),
        b'5' => point2(1, 1),
        b'6' => point2(2, 1),
        b'1' => point2(0, 2),
        b'2' => point2(1, 2),
        b'3' => point2(2, 2),
        b'0' => point2(1, 3),
        b'A' => point2(2, 3),
        other => panic!("Invalid keypad symbol {other}"),
    }
}

// `None` is used for A
fn dpad_pos(d: Move) -> Point2D<i32> {
    match d {
        Some(Vector2D { x: 0, y: -1, .. }) => point2(1, 0),
        None => point2(2, 0),
        Some(Vector2D { x: -1, y: 0, .. }) => point2(0, 1),
        Some(Vector2D { x: 0, y: 1, .. }) => point2(1, 1),
        Some(Vector2D { x: 1, y: 0, .. }) => point2(2, 1),
        other => panic!("Invalid dpad symbol {other:?}"),
    }
}

fn moves_for_diff(diff: Vector2D<i32>, pos: Point2D<i32>, gap: Point2D<i32>) -> Vec<Vec<Move>> {
    let horizontal = iter::repeat_n(
        Some(vec2(diff.x.signum(), 0)),
        diff.x.unsigned_abs() as usize,
    );
    let vertical = iter::repeat_n(
        Some(vec2(0, diff.y.signum())),
        diff.y.unsigned_abs() as usize,
    );
    if pos + vec2(diff.x, 0) == gap {
        // Must not move horizontal first, or we hit the gap
        vec![vertical.chain(horizontal).chain(iter::once(None)).collect()]
    } else if pos + vec2(0, diff.y) == gap {
        vec![horizontal.chain(vertical).chain(iter::once(None)).collect()]
    } else {
        // Try both horizontal first and vertical first
        vec![
            horizontal
                .clone()
                .chain(vertical.clone())
                .chain(iter::once(None))
                .collect(),
            vertical.chain(horizontal).chain(iter::once(None)).collect(),
        ]
    }
}

fn dpad_sequence_len(
    start: Move,
    end: Move,
    rounds: u32,
    cache: &mut FxHashMap<(Move, Move, u32), u64>,
) -> u64 {
    if rounds == 0 {
        return 1;
    }
    if let Some(len) = cache.get(&(start, end, rounds)) {
        return *len;
    }
    let start_pos = dpad_pos(start);
    let end_pos = dpad_pos(end);
    let diff = end_pos - start_pos;
    let possible_paths = moves_for_diff(diff, start_pos, DPAD_GAP);
    let shortest_sequence = possible_paths
        .iter()
        .map(|moves| {
            moves
                .iter()
                .fold((0, None), |(cost, prev), &m| {
                    (cost + dpad_sequence_len(prev, m, rounds - 1, cache), m)
                })
                .0
        })
        .min()
        .unwrap();
    cache.insert((start, end, rounds), shortest_sequence);
    shortest_sequence
}

fn keypad_sequence_len(start: u8, end: u8, rounds: u32) -> u64 {
    let start_pos = keypad_pos(start);
    let end_pos = keypad_pos(end);
    let diff = end_pos - start_pos;
    let possible_paths = moves_for_diff(diff, start_pos, KEYPAD_GAP);
    let mut cache = FxHashMap::default();
    possible_paths
        .iter()
        .map(|moves| {
            moves
                .iter()
                .fold((0, None), |(cost, prev), &m| {
                    (cost + dpad_sequence_len(prev, m, rounds, &mut cache), m)
                })
                .0
        })
        .min()
        .unwrap()
}

fn solve(input: &str, rounds: u32) -> u64 {
    let mut sum: u64 = 0;
    for l in input.lines() {
        let mut prev = b'A';
        let mut len = 0;
        for b in l.bytes() {
            len += keypad_sequence_len(prev, b, rounds);
            prev = b;
        }
        let code_n: u64 = l.strip_suffix('A').unwrap().parse().unwrap();
        sum += code_n * len;
    }
    sum
}

fn part1(input: String) {
    println!("{}", solve(&input, 2));
}

fn part2(input: String) {
    println!("{}", solve(&input, 25));
}

util::aoc_main!();
