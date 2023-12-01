use std::fs;
use std::process::exit;
use std::collections::HashMap;
use std::ops::{Add, Sub};

static INPUT: &str = "input/day22.txt";
const SIDE: usize = 50;

type FaceMap = HashMap<Face, (Vec<Vec<bool>>, (usize, usize), Dir)>;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_dir(&self, turn_right: bool) -> Self {
        match (self, turn_right) {
            (Dir::Right, false) | (Dir::Left, true) => Dir::Up,
            (Dir::Down, false) | (Dir::Up, true) => Dir::Right,
            (Dir::Left, false) | (Dir::Right, true) => Dir::Down,
            (Dir::Up, false) | (Dir::Down, true) => Dir::Left,
        }
    }

    fn apply(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::Up => Some((pos.0.checked_sub(1)?, pos.1)),
            Self::Right => Some((pos.0, pos.1 + 1)),
            Self::Down => Some((pos.0 + 1, pos.1)),
            Self::Left => Some((pos.0, pos.1.checked_sub(1)?)),
        }
    }

    fn num(&self) -> u32 {
        match self {
            Self::Up => 0,
            Self::Right => 1,
            Self::Down => 2,
            Self::Left => 3,
        }
    }

    fn sym(n: u32) -> Self {
        match n {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            3 => Self::Left,
            _ => panic!("Invalid number"),
        }
    }

    fn start(&self, prev_dir: &Self, prev_pos: (usize, usize), reverse: bool) -> (usize, usize) {
        let mut common_coord = match prev_dir {
            Self::Up | Self::Down => prev_pos.1,
            Self::Right | Self::Left => prev_pos.0,
        };
        if reverse {
            common_coord = (SIDE - 1) - common_coord;
        }
        match self {
            Self::Up => (SIDE - 1, common_coord),
            Self::Right => (common_coord, 0),
            Self::Down => (0, common_coord),
            Self::Left => (common_coord, SIDE - 1),
        }
    }
}

impl Add for Dir {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self::sym((self.num() + other.num()) % 4)
    }
}

impl Sub for Dir {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self::sym((self.num() as i32 - other.num() as i32).rem_euclid(4) as u32)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Face {
    Top,
    Back,
    Right,
    Front,
    Left,
    Bottom,
}

impl Face {
    // Return (new face, direction on that face, whether start position is reversed)
    fn change(&self, dir: &Dir) -> (Self, Dir, bool) {
        match (self, dir) {
            // OOOF!
            (Face::Top, Dir::Up) => (Face::Back, Dir::Down, true),
            (Face::Top, Dir::Right) => (Face::Right, Dir::Down, true),
            (Face::Top, Dir::Down) => (Face::Front, Dir::Down, false),
            (Face::Top, Dir::Left) => (Face::Left, Dir::Down, false),

            (Face::Back, Dir::Up) => (Face::Top, Dir::Down, true),
            (Face::Back, Dir::Right) => (Face::Left, Dir::Right, false),
            (Face::Back, Dir::Down) => (Face::Bottom, Dir::Down, false),
            (Face::Back, Dir::Left) => (Face::Right, Dir::Left, false),

            (Face::Right, Dir::Up) => (Face::Top, Dir::Left, true),
            (Face::Right, Dir::Right) => (Face::Back, Dir::Right, false),
            (Face::Right, Dir::Down) => (Face::Bottom, Dir::Right, true),
            (Face::Right, Dir::Left) => (Face::Front, Dir::Left, false),

            (Face::Front, Dir::Up) => (Face::Top, Dir::Up, false),
            (Face::Front, Dir::Right) => (Face::Right, Dir::Right, false),
            (Face::Front, Dir::Down) => (Face::Bottom, Dir::Up, true),
            (Face::Front, Dir::Left) => (Face::Left, Dir::Left, false),

            (Face::Left, Dir::Up) => (Face::Top, Dir::Right, false),
            (Face::Left, Dir::Right) => (Face::Front, Dir::Right, false),
            (Face::Left, Dir::Down) => (Face::Bottom, Dir::Left, false),
            (Face::Left, Dir::Left) => (Face::Back, Dir::Left, false),

            (Face::Bottom, Dir::Up) => (Face::Back, Dir::Up, false),
            (Face::Bottom, Dir::Right) => (Face::Left, Dir::Up, false),
            (Face::Bottom, Dir::Down) => (Face::Front, Dir::Up, true),
            (Face::Bottom, Dir::Left) => (Face::Right, Dir::Up, true),
        }
    }
}

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {e}");
            exit(1);
        });
    let (faces, route) = parse_input(&input);
    //println!("{:?}", &faces);
    let (pos, dir, face) = walk(&faces, route);
    println!("{:?}, {:?}, {:?}", pos, dir, face);
    println!("{}", score(&faces, pos, dir, face));
}

fn score(faces: &FaceMap, pos: (usize, usize), dir: Dir, face: Face) -> usize {
    let (real_pos, real_dir) = real_pos(faces, pos, dir, face);
    let dir_score = match real_dir {
        Dir::Right => 0,
        Dir::Down => 1,
        Dir::Left => 2,
        Dir::Up => 3,
    };
    println!("{:?}, {:?}", real_pos, real_dir);
    (real_pos.0 + 1) * 1000 + (real_pos.1 + 1) * 4 + dir_score
}

fn real_pos(faces: &FaceMap, pos: (usize, usize), dir: Dir, face: Face)
-> ((usize, usize), Dir) {
    let (_grid, fpos, fdir) = &faces[&face];
    // Rotate position back
    let rotated_pos = rotate_pos(pos, *fdir);
    // Apply face offset
    let real_pos = (fpos.0*SIDE + rotated_pos.0, fpos.1*SIDE + rotated_pos.1);
    // Rotate direction back
    let real_dir = dir + *fdir;
    (real_pos, real_dir)
}

fn rotate_pos(pos: (usize, usize), dir: Dir) -> (usize, usize) {
    let n = SIDE - 1;
    match dir {
        Dir::Up => pos,
        Dir::Right => (pos.1, n - pos.0),
        Dir::Down => (n - pos.0, n - pos.1),
        Dir::Left => (n - pos.1, pos.0),
    }
}

fn walk(map: &FaceMap, route: Vec<(u16, Dir)>)
-> ((usize, usize), Dir, Face) {
    // Move right at the beginning
    let mut dir = Dir::Right;
    let mut face = Face::Top;
    let mut pos = (0, 0);
    for (length, turn_dir) in route {
        //println!("{:?}", real_pos(map, pos, dir, face));
        for _ in 0..length {
            //println!("{:?}", (pos, dir, face));
            //println!("{:?}", real_pos(map, pos, dir, face));
            if let Some(new_pos) = dir.apply(pos) {
                if new_pos.0 < SIDE && new_pos.1 < SIDE {
                    // Still within bounds
                    match map[&face].0[new_pos.0][new_pos.1] {
                        // Hit wall, continue to next route element
                        true => break,
                        // Keep going
                        false => { pos = new_pos; continue },
                    }
                }
            }
            // Out of bounds, change face
            let (new_face, new_dir, reverse) = face.change(&dir);
            let new_pos = new_dir.start(&dir, pos, reverse);
            match map[&new_face].0[new_pos.0][new_pos.1] {
                true => break,
                false => {  // Switch to new face
                    pos = new_pos;
                    face = new_face;
                    dir = new_dir;
                },
            }
        }
        dir = dir + turn_dir;
    }
    // Last route element contains a left turn, but the input ends with a move
    dir = dir.turn_dir(true);
    (pos, dir, face)
}

fn parse_input(input: &str) -> (FaceMap, Vec<(u16, Dir)>) {
    let mut rows: Vec<Vec<Option<bool>>> = Vec::new();
    let mut lines_it = input.lines();
    for line in &mut lines_it {
        if line.is_empty() {
            break;
        }
        let row: Vec<Option<bool>> = line.as_bytes().iter()
            .map(|b| match b {
                b' ' => None,
                b'.' => Some(false),
                b'#' => Some(true),
                _ => panic!("Invalid character: {b}"),
            }).collect();
        rows.push(row);
    }
    rows.push(Vec::new());
    // Make all rows have the same width
    let width = rows.iter().map(|r| r.len()).max().unwrap() + 1;
    for row in rows.iter_mut() {
        if row.len() < width {
            row.append(&mut vec![None; width - row.len()]);
        }
    }
    let faces = split_map(&rows);

    (faces, parse_movement(lines_it.next().unwrap()))
}

fn split_map(map: &Vec<Vec<Option<bool>>>) -> FaceMap {
    let full_width = map[0].len();
    let mut faces = HashMap::new();
    let mut pos = (0, 0);
    for i in 0..(full_width/SIDE) {
        if let Some(_) = map[0][i*SIDE] {
            pos = (0, i);
            faces.insert(Face::Top, (extract_face(map, pos, Dir::Up), pos, Dir::Up));
            break;
        }
    }
    next_face(map, &mut faces, Face::Top, Dir::Up, pos);
    faces
}

fn next_face(
    map: &Vec<Vec<Option<bool>>>,
    faces: &mut FaceMap,
    face: Face,
    prev_dir: Dir,
    pos: (usize, usize),
)
{
    let full_height = map.len();
    let full_width = map[0].len();
    for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
        if let Some(new_pos) = dir.apply(pos) {
            if new_pos.0 < full_height && new_pos.1 < full_width &&
                map[new_pos.0*SIDE][new_pos.1*SIDE].is_some() &&
                faces.values().position(|(_, p, _)| *p == new_pos).is_none()
            {
                let (new_face, new_dir, _) = face.change(&(dir - prev_dir));
                let face_dir = dir - new_dir;
                if faces.contains_key(&new_face) {
                    eprintln!("Finding same face twice!!");
                    exit(6);
                }
                faces.insert(new_face, (extract_face(map, new_pos, face_dir), new_pos, face_dir));
                // Recurse
                next_face(map, faces, new_face, face_dir, new_pos);
            }
        }
    }
}

fn extract_face(
    map: &Vec<Vec<Option<bool>>>,
    pos: (usize, usize),
    dir: Dir,
) -> Vec<Vec<bool>> {
    let mut face = Vec::new();
    for row in 0..SIDE {
        face.push(map[pos.0*SIDE + row][(pos.1*SIDE)..((pos.1 + 1)*SIDE)].iter()
            .map(|x| x.expect("Map should contain blocks"))
            .collect());
    }
    // Rotate counterclockwise as often as needed. Terribly unoptimized, but that's what you get.
    for _ in 0..(Dir::Up - dir).num() {
        face = rotate_grid(&face);
    }
    face
}

// Return grid rotated counterclockwise
fn rotate_grid(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut rotated = Vec::new();
    // columns of grid, rows of rotated
    for i in 0..grid[0].len() {
        rotated.push(grid.iter().map(|r| r[i]).rev().collect());
    }
    rotated
}

fn parse_movement(input: &str) -> Vec<(u16, Dir)> {
    let mut moves = Vec::new();
    let mut last_sep: isize = -1;
    for (i, c) in input.chars().enumerate() {
        if !c.is_ascii_digit() {
            let n = input[(last_sep+1) as usize..i].parse().unwrap();
            let dir = match c {
                'R' => Dir::Right,
                'L' => Dir::Left,
                _ => panic!("Character {c} not a direction"),
            };
            moves.push((n, dir));
            last_sep = i as isize;
        }
    }
    let n = input[(last_sep+1) as usize..].parse().unwrap();
    moves.push((n, Dir::Left));
    moves
}
