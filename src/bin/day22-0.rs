use std::fs;
use std::process::exit;

static INPUT: &str = "input/day22.txt";

#[derive(Clone, PartialEq)]
enum Spot {
    Space,
    Wall,
    Void,
}

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {e}");
            exit(1);
        });
    let (map, route) = parse_input(&input);
    let (pos, dir) = walk(map, route);
    println!("{:?}, {:?}", pos, dir);
    println!("{}", score(pos, dir));
}

fn score(pos: (usize, usize), dir: (i8, i8)) -> usize {
    let dir_score = match dir {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => panic!("Invalid direction"),
    };
    (pos.0) * 1000 + (pos.1) * 4 + dir_score
}

fn walk(map: Vec<Vec<Spot>>, route: Vec<(u16, bool)>)
-> ((usize, usize), (i8, i8)) {
    // Move right at the beginning
    let mut dir: (i8, i8) = (0, 1);
    let mut pos = wrap(&map, (1, 1), dir);
    println!("{:?}", pos);
    for (length, turn_right) in route {
        for _ in 0..length {
            let mut new_pos = add_dir(pos, dir);
            if map[new_pos.0][new_pos.1] == Spot::Void {
                new_pos = wrap(&map, pos, dir);
            }
            match map[new_pos.0][new_pos.1] {
                Spot::Space => pos = new_pos,
                Spot::Wall => break,
                Spot::Void => panic!("Should not encounter void anymore"),
            }
        }
        dir = turn_dir(dir, turn_right);
    }
    // Last route element contains a left turn, but the input ends with a move
    dir = turn_dir(dir, true);
    (pos, dir)
}

fn turn_dir(dir: (i8, i8), turn_right: bool) -> (i8, i8) {
    match (dir, turn_right) {
        ((0, 1), false) | ((0, -1), true) => (-1, 0),
        ((1, 0), false) | ((-1, 0), true) => (0, 1),
        ((0, -1), false) | ((0, 1), true) => (1, 0),
        ((-1, 0), false) | ((1, 0), true) => (0, -1),
        _ => panic!("Invalid direction"),
    }
}

fn add_dir(pos: (usize, usize), dir: (i8, i8)) -> (usize, usize) {
    (pos.0.saturating_add_signed(dir.0.into()), pos.1.saturating_add_signed(dir.1.into()))
}

fn wrap(map: &[Vec<Spot>], pos: (usize, usize), dir: (i8, i8)) -> (usize, usize) {
    match dir {
        (-1, 0) => (map.iter().map(|row| &row[pos.1])
                       .rposition(|spot| *spot != Spot::Void).unwrap(),
                    pos.1),
        (1, 0) => (map.iter().map(|row| &row[pos.1])
                      .position(|spot| *spot != Spot::Void).unwrap(),
                   pos.1),
        (0, -1) => (pos.0,
                    map[pos.0].iter()
                       .rposition(|spot| *spot != Spot::Void).unwrap()),
        (0, 1) => (pos.0,
                   map[pos.0].iter()
                       .position(|spot| *spot != Spot::Void).unwrap()),
        _ => panic!("Invalid direction"),
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Spot>>, Vec<(u16, bool)>) {
    let mut rows: Vec<Vec<Spot>> = vec![Vec::new()];
    let mut lines_it = input.lines();
    for line in &mut lines_it {
        if line.is_empty() {
            break;
        }
        let mut row: Vec<Spot> = line.as_bytes().iter()
            .map(|b| match b {
                b' ' => Spot::Void,
                b'.' => Spot::Space,
                b'#' => Spot::Wall,
                _ => panic!("Invalid character: {b}"),
            }).collect();
        row.insert(0, Spot::Void);
        rows.push(row);
    }
    rows.push(Vec::new());
    // Make all rows have the same width
    let width = rows.iter().map(|r| r.len()).max().unwrap() + 1;
    for row in rows.iter_mut() {
        if row.len() < width {
            row.append(&mut vec![Spot::Void; width - row.len()]);
        }
    }

    (rows, parse_movement(lines_it.next().unwrap()))
}

fn parse_movement(input: &str) -> Vec<(u16, bool)> {
    let mut moves = Vec::new();
    let mut last_sep: isize = -1;
    for (i, c) in input.chars().enumerate() {
        if !c.is_ascii_digit() {
            let n = input[(last_sep+1) as usize..i].parse().unwrap();
            moves.push((n, c == 'R'));
            last_sep = i as isize;
        }
    }
    let n = input[(last_sep+1) as usize..].parse().unwrap();
    moves.push((n, false));
    moves
}
