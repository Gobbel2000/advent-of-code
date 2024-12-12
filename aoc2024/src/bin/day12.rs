use std::collections::{HashSet, VecDeque};

use euclid::{default::*, point2, vec2};

type Fences = HashSet<(Point2D<i32>, Point2D<i32>)>;
const DIRS: [Vector2D<i32>; 4] = [vec2(0, -1), vec2(1, 0), vec2(0, 1), vec2(-1, 0)];

fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|l| l.as_bytes()).collect()
}

fn price(field: &[&[u8]], start: (usize, usize), visited: &mut [Vec<bool>]) -> (u32, Fences) {
    let crop = field[start.1][start.0];
    let width = field[0].len();
    let height = field.len();
    let mut area_visited = vec![vec![false; width]; height];
    let mut area = 0;
    let mut fences: Fences = HashSet::new();

    area_visited[start.1][start.0] = true;
    visited[start.1][start.0] = true;
    let start = point2(start.0 as i32, start.1 as i32);
    let bounds = Rect::new(Point2D::origin(), Size2D::new(width, height).to_i32());
    let mut frontier = VecDeque::from([start]);
    while let Some(p) = frontier.pop_front() {
        area += 1;
        for dir in DIRS {
            let next = p + dir;
            if bounds.contains(next) {
                let next_u = next.to_usize();
                if area_visited[next_u.y][next_u.x] {
                    continue;
                }
                if field[next_u.y][next_u.x] == crop {
                    visited[next_u.y][next_u.x] = true;
                    area_visited[next_u.y][next_u.x] = true;
                    frontier.push_back(next);
                    continue;
                }
            }
            fences.insert((p, next));
        }
    }
    (area, fences)
}

fn part1(input: String) {
    let field = parse(&input);
    let width = field[0].len();
    let height = field.len();
    let mut visited = vec![vec![false; width]; height];
    let mut total_price = 0;
    for y in 0..height {
        for x in 0..width {
            if !visited[y][x] {
                let (area, fences) = price(&field, (x, y), &mut visited);
                total_price += area * fences.len() as u32;
            }
        }
    }
    println!("{total_price}");
}

fn count_perimeter(mut fences: Fences) -> u32 {
    let list: Vec<_> = fences.iter().copied().collect();
    let mut perimeter = 0;
    for (v, w) in list {
        if fences.contains(&(v, w)) {
            perimeter += 1;
            let dir = w - v;
            let orth = dir.yx();
            let mut next = v + orth;
            while fences.remove(&(next, next + dir)) {
                next += orth;
            }
            let mut next = v - orth;
            while fences.remove(&(next, next + dir)) {
                next -= orth;
            }
        }
    }
    perimeter
}

fn part2(input: String) {
    let field = parse(&input);
    let width = field[0].len();
    let height = field.len();
    let mut visited = vec![vec![false; width]; height];
    let mut total_price = 0;
    for y in 0..height {
        for x in 0..width {
            if !visited[y][x] {
                let (area, fences) = price(&field, (x, y), &mut visited);
                total_price += area * count_perimeter(fences);
            }
        }
    }
    println!("{total_price}");
}

util::aoc_main!();
