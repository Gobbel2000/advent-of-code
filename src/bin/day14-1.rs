use std::fs;
use std::process::exit;
use std::cmp::{min, max};

static INPUT: &str = "input/day14.txt";
const SAND_START: (i32, i32) = (500, 0);

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {}", e);
            exit(1);
        });
    let cave = parse_input(&input).unwrap_or_else(|| {
        eprintln!("Malformed file");
        exit(2);
    });
    let mut grid = Grid::to_grid(cave);
    //println!("{:?}", grid);
    let mut counter = 0;
    while grid.add_sand() {
        counter += 1;
    }
    println!("{}", counter);
}

fn parse_input(input: &str) -> Option<Vec<Vec<(i32, i32)>>> {
    let mut cave = Vec::new();
    for line in input.lines() {
        let mut path = Vec::new();
        for coord in line.split(" -> ") {
            let mut values = coord.split(',').map(|c| c.parse().ok());
            path.push((values.next()??, values.next()??));
        }
        cave.push(path);
    }
    return Some(cave);
}

#[derive(Debug)]
struct Grid {
    array: Vec<Vec<bool>>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Grid {
    // Creates an empty grid with the correct size for the cave
    fn make_array(cave: &Vec<Vec<(i32, i32)>>) -> Self {
        let mut min_x = SAND_START.0;
        let mut max_x = SAND_START.0;
        let mut min_y = SAND_START.1;
        let mut max_y = SAND_START.1;
        for (x, y) in cave.iter().flatten().copied() {
            min_x = min(x, min_x);
            max_x = max(x, max_x);
            min_y = min(y, min_y);
            max_y = max(y, max_y);
        }
        // Extend y by 2 for the bottom rock platform
        max_y += 2;
        let height = (max_y + 1 - min_y) as usize;
        // Make space on either side for the sand pyramid: It can be at most 2*height wide
        min_x = min(min_x, SAND_START.0 - height as i32);
        max_x = max(min_x, SAND_START.0 + height as i32);
        let width = (max_x + 1 - min_x) as usize;
        let array = vec![vec![false; width]; height];
        let grid = Grid {
            array,
            min_x,
            max_x,
            min_y,
            max_y,
        };
        return grid;
    }

    fn get(&self, pos: (i32, i32)) -> bool {
        self.array[(pos.1 - self.min_y) as usize][(pos.0 - self.min_x) as usize]
    }

    fn set(&mut self, pos: (i32, i32), val: bool) {
        self.array[(pos.1 - self.min_y) as usize][(pos.0 - self.min_x) as usize] = val;
    }

    fn to_grid(cave: Vec<Vec<(i32, i32)>>) -> Self {
        let mut grid = Self::make_array(&cave);
        for path in cave {
            for i in 0..(path.len() - 1) {
                let start = path[i];
                let end = path[i+1];
                // Horizontal line
                if start.0 == end.0 {
                    for y in min(start.1, end.1)..=max(start.1, end.1) {
                        grid.set((start.0, y), true);
                    }
                } else if start.1 == end.1 {
                    for x in min(start.0, end.0)..=max(start.0, end.0) {
                        grid.set((x, start.1), true);
                    }
                } else {
                    eprintln!("Line is not axis-parallel");
                    exit(3);
                }
            }
        }
        // Rock bottom
        for x in grid.min_x..=grid.max_x {
            grid.set((x, grid.max_y), true);
        }
        return grid;
    }

    // Add one unit of sand, return false once the inlet is clogged
    fn add_sand(&mut self) -> bool {
        let mut pos = SAND_START;
        if self.get(pos) {
            return false;
        }
        let directions = [(0, 1), (-1, 1), (1, 1)];
        'outer: loop {
            for dir in directions {
                let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if !self.get(new_pos) {
                    // Next position found
                    pos = new_pos;
                    continue 'outer;
                }
            }
            // No new position found, sand is settled
            self.set(pos, true);
            return true;
        }
    }
}
