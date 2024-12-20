use euclid::{default::*, vec2};

const DIRS: [Vector2D<i32>; 4] = [vec2(1, 0), vec2(0, 1), vec2(-1, 0), vec2(0, -1)];
const MIN_SAVE: u32 = 100;
const MAX_DIST: i32 = 20;

fn parse(input: &str) -> (Vec<Vec<bool>>, Point2D<i32>, Point2D<i32>) {
    let mut start = None;
    let mut end = None;
    let mut field = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, b) in line.bytes().enumerate() {
            row.push(b == b'#');
            if b == b'S' {
                start = Some(Point2D::new(x, y).to_i32());
            } else if b == b'E' {
                end = Some(Point2D::new(x, y).to_i32());
            }
        }
        field.push(row);
    }
    (field, start.unwrap(), end.unwrap())
}

fn distances(
    field: &[Vec<bool>],
    start: Point2D<i32>,
    end: Point2D<i32>,
) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let width = field[0].len();
    let height = field.len();
    let mut dist_to_start = vec![vec![u32::MAX; width]; height];
    let bounds = Rect::new(Point2D::origin(), Size2D::new(width, height)).to_i32();

    let mut cur = start;
    let mut dist = 0;
    dist_to_start[cur.y as usize][cur.x as usize] = dist;
    while cur != end {
        for dir in DIRS {
            let next = cur + dir;
            if bounds.contains(next)
                && !field[next.y as usize][next.x as usize]
                && dist_to_start[next.y as usize][next.x as usize] == u32::MAX
            {
                cur = next;
                break;
            }
        }
        dist += 1;
        dist_to_start[cur.y as usize][cur.x as usize] = dist;
    }
    let total_dist = dist_to_start[end.y as usize][end.x as usize];
    let dist_to_end = dist_to_start
        .iter()
        .map(|row| {
            row.iter()
                .map(|&d| {
                    if d == u32::MAX {
                        u32::MAX
                    } else {
                        total_dist - d
                    }
                })
                .collect()
        })
        .collect();
    (dist_to_start, dist_to_end)
}

fn cheats(
    field: &[Vec<bool>],
    dist_to_start: &[Vec<u32>],
    dist_to_end: &[Vec<u32>],
    total_dist: u32,
) -> u32 {
    let width = field[0].len();
    let height = field.len();
    let bounds = Rect::new(Point2D::origin(), Size2D::new(width, height)).to_i32();
    let mut count = 0;
    for (y, row) in field.iter().enumerate() {
        for (x, _w) in row.iter().enumerate().filter(|&(_i, w)| *w) {
            let pos = Point2D::new(x, y).to_i32();
            for (d0, &dir0) in DIRS.iter().enumerate().skip(1) {
                for &dir1 in DIRS.iter().take(d0) {
                    let p0 = pos + dir0;
                    let p1 = pos + dir1;
                    if bounds.contains(p0) && bounds.contains(p1) {
                        let p0 = p0.to_usize();
                        let p1 = p1.to_usize();
                        if !field[p0.y][p0.x] && !field[p1.y][p1.x] {
                            let dist = dist_to_start[p0.y][p0.x].min(dist_to_start[p1.y][p1.x])
                                + dist_to_end[p1.y][p1.x].min(dist_to_end[p0.y][p0.x])
                                + 2; // Add 2 for cutting across the wall
                            if total_dist - dist >= MIN_SAVE {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

fn part1(input: String) {
    let (field, start, end) = parse(&input);
    let (dist_to_start, dist_to_end) = distances(&field, start, end);
    let total_dist = dist_to_start[end.y as usize][end.x as usize];
    println!(
        "{}",
        cheats(&field, &dist_to_start, &dist_to_end, total_dist)
    );
}

// Half of all vectors with manhattan distance <= MAX_DIST.
// Only vectors with positive x or going straight down are considered to avoid using the same
// vector twice in both directions.
fn cheat_vectors() -> Vec<Vector2D<i32>> {
    let mut vectors = Vec::new();
    for y in -MAX_DIST..=MAX_DIST {
        let start = if y > 0 { 0 } else { 1 };
        for x in start..=(MAX_DIST - y.abs()) {
            assert!(x + y <= MAX_DIST);
            vectors.push(vec2(x, y));
        }
    }
    vectors
}

fn cheats20(
    field: &[Vec<bool>],
    dist_to_start: &[Vec<u32>],
    dist_to_end: &[Vec<u32>],
    total_dist: u32,
) -> u32 {
    let vectors = cheat_vectors();
    let width = field[0].len();
    let height = field.len();
    let bounds = Rect::new(Point2D::origin(), Size2D::new(width, height)).to_i32();
    let mut count = 0;
    for (y, row) in field.iter().enumerate() {
        for (x, _w) in row.iter().enumerate().filter(|&(_i, w)| !*w) {
            let p0 = Point2D::new(x, y);
            for &v in &vectors {
                let pi1 = p0.to_i32() + v;
                if bounds.contains(pi1) {
                    let p1 = pi1.to_usize();
                    if !field[p1.y][p1.x] {
                        let dist = dist_to_start[p0.y][p0.x].min(dist_to_start[p1.y][p1.x])
                            + dist_to_end[p1.y][p1.x].min(dist_to_end[p0.y][p0.x])
                            + v.x.unsigned_abs()  // Manhattan distance of vector
                            + v.y.unsigned_abs();
                        if total_dist - dist >= MIN_SAVE {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

fn part2(input: String) {
    let (field, start, end) = parse(&input);
    let (dist_to_start, dist_to_end) = distances(&field, start, end);
    let total_dist = dist_to_start[end.y as usize][end.x as usize];
    println!(
        "{}",
        cheats20(&field, &dist_to_start, &dist_to_end, total_dist)
    );
}

util::aoc_main!();
