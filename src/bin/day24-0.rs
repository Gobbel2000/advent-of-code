use std::fs;
use std::process::exit;
use std::cmp::Ordering;

use priority_queue::PriorityQueue;

static INPUT: &str = "input/day24.txt";

#[derive(Clone)]
struct Blizzard {
    pos: (usize, usize),
    dir: (i8, i8),
}

impl Blizzard {
    fn next_pos(&self, bounds: (usize, usize)) -> (usize, usize) {
        ((self.pos.0 as isize + self.dir.0 as isize).rem_euclid(bounds.0 as isize) as usize,
         (self.pos.1 as isize + self.dir.1 as isize).rem_euclid(bounds.1 as isize) as usize)
    }
}

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {e}");
            exit(1);
        });
    let (mut blizzards, pos) = parse_input(&input).unwrap_or_else(|| {
        eprintln!("Error while parsing file");
        exit(2);
    });
    //_print_grid(&blizzards);
    let end = (blizzards.len() - 1, blizzards[0].len() - 1);
    let mut time = 0;
    while !blizzards[0][0].is_empty() {
        blizzards = advance_blizzards(&blizzards);
        time += 1;
    }
    let rounds = astar(&mut blizzards, pos, end).expect("Should have a route");

    println!("{}", rounds + time);
}

fn astar(grid: &Vec<Vec<Vec<Blizzard>>>, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
    let mut pq = PriorityQueue::new();
    pq.push((start, 0), OrdF64(dist(start, end) * -1.0));
    // State of blizzards for each timestamp
    let mut grids = vec![grid.clone()];
    while let Some(((v, i), _prio)) = pq.pop() {
        if v == end {
            return Some(i);
        }
        if grids.len() == i as usize {
            let new_grid = advance_blizzards(&grids[grids.len() - 1]);
            grids.push(new_grid);
        }
        let cur_grid = &grids[i as usize];
        for w in get_options(cur_grid, v) {
            pq.push((w, i + 1), OrdF64(((i + 1) as f64 + dist(w, end)) * -1.0));
        }
    }
    None
}

fn _print_grid(grid: &Vec<Vec<Vec<Blizzard>>>) {
    println!();
    for row in grid {
        for spot in row {
            if spot.is_empty() {
                print!(".");
            } else if spot.len() > 1 {
                print!("{}", spot.len());
            } else {
                match spot[0].dir {
                    (-1, 0) => print!("^"),
                    (0, 1) => print!(">"),
                    (1, 0) => print!("v"),
                    (0, -1) => print!("<"),
                    _ => panic!(),
                }
            }
        }
        println!();
    }
}

fn advance_blizzards(grid: &Vec<Vec<Vec<Blizzard>>>) -> Vec<Vec<Vec<Blizzard>>> {
    let bounds = (grid.len(), grid[0].len());
    let mut new_grid = vec![vec![Vec::<Blizzard>::new(); bounds.1]; bounds.0];
    for spot in grid.iter().flatten() {
        for bliz in spot {
            let new_pos = bliz.next_pos(bounds);
            let mut new_bliz = bliz.clone();
            new_bliz.pos = new_pos;
            new_grid[new_pos.0][new_pos.1].push(new_bliz);
        }
    }
    new_grid
}

fn get_options(grid: &Vec<Vec<Vec<Blizzard>>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let bounds = (grid.len(), grid[0].len());
    let mut options = Vec::new();
    for d in [(0, 0), (-1, 0), (0, 1), (1, 0), (0, -1)] {
        let new_pos = (pos.0 as isize + d.0, pos.1 as isize + d.1);
        if new_pos.0 >= 0 && (new_pos.0 as usize) < bounds.0 &&
            new_pos.1 >= 0 && (new_pos.1 as usize) < bounds.1 &&
            grid[new_pos.0 as usize][new_pos.1 as usize].is_empty()
        {
            options.push((new_pos.0 as usize, new_pos.1 as usize));
        }
    }
    options
}

fn parse_input(input: &str) -> Option<(Vec<Vec<Vec<Blizzard>>>, (usize, usize))> {
    let mut lines = input.lines();
    let x0 = lines.next()?.as_bytes().iter().position(|b| *b == b'.')? - 1;
    let start_pos = (0, x0);

    let mut grid = Vec::new();
    'rows: for (row, line) in lines.enumerate() {
        let mut bliz_row = Vec::new();
        for (col, spot) in line.get(1..(line.len()-1))?.as_bytes().iter().enumerate() {
            let pos = (row, col);
            match spot {
                b'.' => bliz_row.push(Vec::new()),
                b'^' => bliz_row.push(vec![Blizzard { pos, dir: (-1, 0) }]),
                b'>' => bliz_row.push(vec![Blizzard { pos, dir: (0, 1) }]),
                b'v' => bliz_row.push(vec![Blizzard { pos, dir: (1, 0) }]),
                b'<' => bliz_row.push(vec![Blizzard { pos, dir: (0, -1) }]),
                b'#' => break 'rows,
                _ => return None,
            }
        }
        grid.push(bliz_row);
    }

    Some((grid, start_pos))
}

fn dist(a: (usize, usize), b: (usize, usize)) -> f64 {
    (((b.0 as i32 - a.0 as i32).pow(2) +
      (b.1 as i32 - a.1 as i32).pow(2)) as f64).sqrt()
}

// f64 with Ord, to work as a priority with PriorityQueue
#[derive(PartialOrd, PartialEq)]
struct OrdF64(f64);

impl Eq for OrdF64 {}

impl Ord for OrdF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => Ordering::Less,
        }
    }
}
