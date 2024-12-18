use std::collections::VecDeque;

use euclid::{default::*, vec2};

fn parse(input: &str) -> Vec<Point2D<i32>> {
    input
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            Point2D::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

const BOUNDS: Rect<i32> = Rect::new(Point2D::new(0, 0), Size2D::new(71, 71));
const START: Point2D<i32> = Point2D::new(0, 0);
const TARGET: Point2D<i32> = Point2D::new(70, 70);
const N_BYTES: usize = 1024;
const DIRS: [Vector2D<i32>; 4] = [vec2(1, 0), vec2(0, 1), vec2(-1, 0), vec2(0, -1)];

fn adj(
    field: &[[bool; BOUNDS.size.width as usize]],
    v: Point2D<i32>,
) -> impl Iterator<Item = Point2D<i32>> + use<'_> {
    DIRS.iter()
        .map(move |&d| v + d)
        .filter(|&next| BOUNDS.contains(next) && !field[next.y as usize][next.x as usize])
}

fn find_path(field: &[[bool; BOUNDS.size.width as usize]]) -> Option<u32> {
    let mut seen = [[false; BOUNDS.size.width as usize]; BOUNDS.size.height as usize];
    let mut q = VecDeque::from([(START, 0)]);
    seen[START.y as usize][START.x as usize] = true;
    while let Some((v, dist)) = q.pop_front() {
        for w in adj(field, v) {
            if w == TARGET {
                return Some(dist + 1);
            }
            if !seen[w.y as usize][w.x as usize] {
                seen[w.y as usize][w.x as usize] = true;
                q.push_back((w, dist + 1));
            }
        }
    }
    None
}

fn part1(input: String) {
    let bytes = parse(&input);
    let mut field = [[false; BOUNDS.size.width as usize]; BOUNDS.size.height as usize];
    for b in &bytes[..N_BYTES] {
        field[b.y as usize][b.x as usize] = true;
    }
    println!("{}", find_path(&field).unwrap());
}

fn part2(input: String) {
    let bytes = parse(&input);
    let mut field = [[false; BOUNDS.size.width as usize]; BOUNDS.size.height as usize];
    for (i, b) in bytes.iter().enumerate() {
        field[b.y as usize][b.x as usize] = true;
        // We already know from part 1 that below N_BYTES there is a path
        if i > N_BYTES && find_path(&field).is_none() {
            println!("{},{}", b.x, b.y);
            break;
        }
    }
}

util::aoc_main!();
