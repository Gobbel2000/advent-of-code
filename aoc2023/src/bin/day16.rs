use std::collections::VecDeque;
use ndarray::Array2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N = 1,
    E = 2,
    S = 3,
    W = 4,
}

use Dir::*;

impl Dir {
    fn apply(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            N => (pos.0.wrapping_sub(1), pos.1),
            E => (pos.0, pos.1 + 1),
            S => (pos.0 + 1, pos.1),
            W => (pos.0, pos.1.wrapping_sub(1)),
        }
    }
}


#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Mirror {
    #[default]
    Empty,
    MirrorRight,     //  /
    MirrorLeft,      //  \
    SplitVertical,   //  | 
    SplitHorizontal, // -
}

impl Mirror {
    fn redirect(&self, dir: Dir) -> Vec<Dir> {
        match (self, dir) {
            (Mirror::Empty, _) => vec![dir],
            (Mirror::MirrorRight, E) => vec![N],
            (Mirror::MirrorRight, S) => vec![W],
            (Mirror::MirrorRight, N) => vec![E],
            (Mirror::MirrorRight, W) => vec![S],

            (Mirror::MirrorLeft, E) => vec![S],
            (Mirror::MirrorLeft, N) => vec![W],
            (Mirror::MirrorLeft, S) => vec![E],
            (Mirror::MirrorLeft, W) => vec![N],

            (Mirror::SplitVertical, N | S) => vec![dir],
            (Mirror::SplitVertical, E | W) => vec![N, S],
            (Mirror::SplitHorizontal, E | W) => vec![dir],
            (Mirror::SplitHorizontal, N | S) => vec![E, W],
        }
    }
}

impl From<u8> for Mirror {
    fn from(c: u8) -> Mirror {
        match c {
            b'/' => Mirror::MirrorRight,
            b'\\' => Mirror::MirrorLeft,
            b'|' => Mirror::SplitVertical,
            b'-' => Mirror::SplitHorizontal,
            _ => Mirror::Empty,
        }
    }
}

fn read_input(input: String) -> Array2<Mirror> {
    let lines: Vec<_> = input.lines().collect();
    let n_lines = lines.len();
    let elements: Vec<_> = lines.iter()
        .flat_map(|l| l.bytes().map(Mirror::from))
        .collect();
    Array2::from_shape_vec((n_lines, elements.len() / n_lines), elements)
        .unwrap()
}

fn beam(field: &Array2<Mirror>, start_pos: (usize, usize), dir: Dir) -> u32 {
    let mut beams = VecDeque::from([(start_pos, dir)]);
    // For counting unique energized fields
    let mut energized = vec![false; field.len()];
    // For finding cycles, directions are also included in the key
    let mut positions = vec![false; field.len() * 4];
    while let Some((pos, dir)) = beams.pop_front() {
        if let Some(mirror) = field.get(pos) {
            let fkey = pos.0 * field.ncols() + pos.1;
            energized[fkey] = true;
            let key = (pos.0 * field.ncols() + pos.1) * 4 + dir as usize;
            if positions[key] {
                // Cycle detected, stop looking at this beam
                continue;
            } else {
                positions[key] = true;
            }
            for new_dir in mirror.redirect(dir) {
                let new_pos = new_dir.apply(pos);
                beams.push_back((new_pos, new_dir));
            }
        }
    }
    energized.iter().filter(|&e| *e).count() as u32
}

fn part1(input: String) {
    let field = read_input(input);
    let energized = beam(&field, (0, 0), E);
    println!("{}", energized);
}

fn part2(input: String) {
    let field = read_input(input);
    let mut max = 0;
    let n = field.nrows();
    // Input is quadratic
    assert!(n == field.ncols());
    for i in 0..n {
        max = max.max(beam(&field, (i, 0), E));   // Left edge
        max = max.max(beam(&field, (0, i), S));   // Top edge
        max = max.max(beam(&field, (i, n), W));   // Right edge
        max = max.max(beam(&field, (n, i), N));   // Bottom edge
    }
    println!("{max}");
}

util::aoc_main!("day16.txt");
