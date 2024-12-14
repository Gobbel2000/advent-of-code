use euclid::default::*;
use regex::Regex;

fn parse(input: &str) -> Vec<(Point2D<i32>, Vector2D<i32>)> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            let (_, [p0, p1, v0, v1]) = cap.extract();
            (
                Point2D::new(p0.parse().unwrap(), p1.parse().unwrap()),
                Vector2D::new(v0.parse().unwrap(), v1.parse().unwrap()),
            )
        })
        .collect()
}

const ROOM: Size2D<i32> = Size2D::new(101, 103);
const TIME: i32 = 100;

fn part1(input: String) {
    let robots = parse(&input);
    let new_pos: Vec<Point2D<i32>> = robots.iter()
        .map(|&(p, v)| (p + v * TIME).rem_euclid(&ROOM))
        .collect();

    assert_eq!(ROOM.width % 2, 1);
    assert_eq!(ROOM.height % 2, 1);
    let mid_x = ROOM.width / 2;
    let mid_y = ROOM.height / 2;
    
    let mut q = [0u32; 4];
    for p in new_pos {
        use std::cmp::Ordering::*;
        match (p.x.cmp(&mid_x), p.y.cmp(&mid_y)) {
            (Less, Less) => q[0] += 1,
            (Greater, Less) => q[1] += 1,
            (Less, Greater) => q[2] += 1,
            (Greater, Greater) => q[3] += 1,
            _ => {}
        };
    }
    let prod = q[0] * q[1] * q[2] * q[3];
    println!("{prod}");
}

fn print_map(map: &[Vec<bool>]) {
    for row in map {
        for p in row {
            if *p { print!("#") } else { print!(".") }
        }
        println!();
    }
    println!();
}


fn part2(input: String) {
    let mut robots = parse(&input);
    let mut map = vec![vec![false; ROOM.width as usize]; ROOM.height as usize];
    for i in 1.. {
        let mut overlap = false;
        for (p, v) in &mut robots {
            *p = (*p + *v).rem_euclid(&ROOM);
            if map[p.y as usize][p.x as usize] {
                // Found two robots on the same spot,
                // which is used as a heuristic for detecting the easter egg.
                overlap = true;
            } else {
                map[p.y as usize][p.x as usize] = true;
            }
        }
        if !overlap {
            print_map(&map);
            println!("Round: {i}");
            break;
        }
        for row in &mut map {
            row.fill(false);
        }
    }
}

util::aoc_main!();
