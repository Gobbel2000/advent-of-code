use euclid::default::{Point2D, Vector2D};
use euclid::vec2;

fn parse(input: String) -> (Vec<Vec<bool>>, Point2D<i32>) {
    let mut field = Vec::new();
    let mut start = Point2D::zero();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(c == '#');
            if c == '^' {
                start = Point2D::new(x, y).to_i32();
            }
        }
        field.push(row);
    }
    (field, start)
}

const DIRS: [Vector2D<i32>; 4] = [vec2(0, -1), vec2(1, 0), vec2(0, 1), vec2(-1, 0)];

fn visited(field: &[Vec<bool>], start: Point2D<i32>) -> Vec<Vec<bool>> {
    let width = field[0].len();
    let height = field.len();
    let mut visited = vec![vec![false; width]; height];
    // Start up, then turn right
    let mut dir = 0;
    let mut pos = start;
    loop {
        visited[pos.y as usize][pos.x as usize] = true;
        let next = pos + DIRS[dir];
        // Guard leaves area
        if next.x < 0 || next.y < 0 || next.x >= width as i32 || next.y >= height as i32 {
            break;
        }
        // Path blocked
        if field[next.y as usize][next.x as usize] {
            dir = (dir + 1) % 4; // Turn right, don't move yet
        } else {
            pos = next
        }
    }
    visited
}

fn part1(input: String) {
    let (field, start) = parse(input);
    let count = visited(&field, start)
        .iter()
        .map(|r| r.iter().map(|b| u32::from(*b)).sum::<u32>())
        .sum::<u32>();
    println!("{count}")
}

fn is_loop(field: &[Vec<bool>], start: Point2D<i32>) -> bool {
    let width = field[0].len();
    let height = field.len();
    let mut visited = vec![vec![0; width]; height];

    // Start up, then turn right
    let mut dir = 0;
    let mut pos = start;
    loop {
        // Loop detected
        if visited[pos.y as usize][pos.x as usize] & (1 << dir) > 0 {
            break true;
        }
        // Record all walked directions at all fields
        visited[pos.y as usize][pos.x as usize] |= 1 << dir;
        let next = pos + DIRS[dir];
        // Guard leaves area
        if next.x < 0 || next.y < 0 || next.x >= width as i32 || next.y >= height as i32 {
            break false;
        }
        // Path blocked
        if field[next.y as usize][next.x as usize] {
            dir = (dir + 1) % 4 // Turn right, don't move yet
        } else {
            pos = next
        }
    }
}

fn part2(input: String) {
    let (mut field, start) = parse(input);
    let width = field[0].len();
    let height = field.len();
    let normal_visited = visited(&field, start); // Part 1 solution
    let mut count = 0;
    for x in 0..width {
        for y in 0..height {
            // Only check places that are visited without any obstacles, and don't check start
            if normal_visited[y][x] && !(x as i32 == start.x && y as i32 == start.y) {
                field[y][x] = true; // Set obstacle
                count += is_loop(&field, start) as u32;
                field[y][x] = false; // Remove obstacle
            }
        }
    }
    println!("{count}");
}

util::aoc_main!();
