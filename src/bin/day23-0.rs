use std::fs;
use std::process::exit;

use fxhash::{FxHashSet, FxHashMap};

static INPUT: &str = "input/day23.txt";
const ROUNDS: u32 = 10;

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir {
    fn adj(&self) -> (Self, Self) {
        match self {
            Dir::N => (Dir::NW, Dir::NE),
            Dir::E => (Dir::NE, Dir::SE),
            Dir::S => (Dir::SE, Dir::SW),
            Dir::W => (Dir::SW, Dir::NW),
            _ => panic!("Not a cardinal direction"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn move_by(&self, dir: Dir) -> Self {
        match dir {
            Dir::N => Self { y: self.y - 1, ..*self },
            Dir::NE => Self { x: self.x + 1, y: self.y - 1 },
            Dir::E => Self { x: self.x + 1, ..*self },
            Dir::SE => Self { x: self.x + 1, y: self.y + 1 },
            Dir::S => Self { y: self.y + 1, ..*self},
            Dir::SW => Self { x: self.x - 1, y: self.y + 1 },
            Dir::W => Self { x: self.x - 1, ..*self},
            Dir::NW => Self { x: self.x - 1, y: self.y - 1 },
        }
    }

    fn in_direction(&self, dir: Dir) -> Vec<Self> {
        let (diag1, diag2) = dir.adj();
        vec![self.move_by(diag1), self.move_by(dir), self.move_by(diag2)]
    }
}

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {e}");
            exit(1);
        });
    let mut positions = parse_input(&input);
    shuffle(&mut positions);
    let n_elves = positions.len() as u32;
    println!("{}", get_rect(&positions) - n_elves);
}

// Return the area the elves occupy
fn get_rect(positions: &FxHashSet<Pos>) -> u32 {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for p in positions.iter() {
        min_x = min_x.min(p.x);
        max_x = max_x.max(p.x);
        min_y = min_y.min(p.y);
        max_y = max_y.max(p.y);
    }
    // Print positions
    /*
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if positions.contains(&Pos { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    } */
    (max_x + 1 - min_x) as u32 * (max_y + 1 - min_y) as u32
}

fn shuffle(positions: &mut FxHashSet<Pos>) {
    let mut directions = vec![Dir::N, Dir::S, Dir::W, Dir::E];
    for _ in 0..ROUNDS {
        let mut proposed: FxHashMap<Pos, Pos> = FxHashMap::default();
        let mut endpoints: FxHashMap<Pos, u32> = FxHashMap::default();
        // Propose next positions
        for p in positions.iter() {
            // First, check if the surrounding space is vacant
            if [Dir::N, Dir::NE, Dir::E, Dir::SE, Dir::S, Dir::SW, Dir::W, Dir::NW].into_iter()
                .any(|d| positions.contains(&p.move_by(d)))
            {
                // Check each cardinal direction
                for card in directions.iter() {
                    if p.in_direction(*card).iter().all(|pos| !positions.contains(pos)) {
                        let endpoint = p.move_by(*card);
                        proposed.insert(*p, endpoint);
                        *endpoints.entry(endpoint).or_insert(0) += 1;
                        break;
                    }
                }
            }
        }
        // Apply propositions
        for (start, end) in proposed.iter() {
            if endpoints[end] == 1 {
                positions.remove(start);
                positions.insert(*end);
            }
        }
        directions.rotate_left(1);
    }
}

fn parse_input(input: &str) -> FxHashSet<Pos> {
    let mut positions = FxHashSet::default();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'#' => positions.insert(Pos { x: col as i32, y: row as i32}),
                b'.' => continue,
                _ => {
                    eprintln!("Invalid character: {c}");
                    exit(2);
                },
            };
        }
    }
    positions
}
