use std::fs;
use std::process::exit;

use fxhash::{FxHashSet, FxHashMap};

static INPUT: &str = "input/day23.txt";

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
    let rounds = shuffle(&mut positions);
    println!("{rounds}");
}

fn shuffle(positions: &mut FxHashSet<Pos>) -> u32 {
    let mut directions = vec![Dir::N, Dir::S, Dir::W, Dir::E];
    let mut proposed: FxHashMap<Pos, Pos> = FxHashMap::default();
    let mut endpoints: FxHashMap<Pos, u32> = FxHashMap::default();
    for i in 0.. {
        // Propose next positions
        for p in positions.iter() {
            // First, check if the surrounding space is vacant
            if [Dir::N, Dir::NE, Dir::E, Dir::SE, Dir::S, Dir::SW, Dir::W, Dir::NW].iter()
                .any(|d| positions.contains(&p.move_by(*d)))
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
        // Noone needs to move, we have converged
        if proposed.is_empty() {
            return i + 1;
        }
        // Apply propositions
        for (start, end) in proposed.iter() {
            if endpoints[end] == 1 {
                positions.remove(start);
                positions.insert(*end);
            }
        }
        directions.rotate_left(1);
        proposed.clear();
        endpoints.clear();
    }
    unreachable!("Escaped out of infinite loop")
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
