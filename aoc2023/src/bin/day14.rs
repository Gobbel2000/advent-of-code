use std::fmt;
use std::collections::HashMap;

use ndarray::{Array2, Axis};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
enum Rock {
    #[default]
    None,
    Cube,
    Round,
}

impl From<char> for Rock {
    fn from(c: char) -> Rock {
        match c {
            '#' => Rock::Cube,
            'O' => Rock::Round,
            _ => Rock::None,
        }
    }
}

impl fmt::Debug for Rock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rock::None => write!(f, "."),
            Rock::Cube => write!(f, "#"),
            Rock::Round => write!(f, "O"),
        }
    }
}

fn read_input(input: String) -> Array2<Rock> {
    let lines: Vec<_> = input.lines().collect();
    let n_lines = lines.len();
    let elements: Vec<_> = lines.iter()
        .flat_map(|l| l.chars().map(Rock::from))
        .collect();
    Array2::from_shape_vec((n_lines, elements.len() / n_lines), elements)
        .unwrap()
}

fn tilt(field: &Array2<Rock>, axis: Axis, pos: bool) -> Array2<Rock> {
    let mut tilted = Array2::default(field.raw_dim());
    let stop = if pos { field.len_of(axis) - 1 } else { 0 };
    tilted.index_axis_mut(axis, stop).assign(&field.index_axis(axis, stop));
    for mut i in 0..field.len_of(axis) {
        if pos {
            i = field.len_of(axis) - i - 1;
        }
        let lane = field.index_axis(axis, i);
        for (j, &rock) in lane.iter().enumerate() {
            let roll_to = if rock == Rock::Round {
                if pos {
                    (i..stop)
                        .find(|&k| tilted[axis_idx(axis, k + 1, j)] != Rock::None)
                        .unwrap_or(stop)
                } else {
                    (1..=i).rev()
                        .find(|&k| tilted[axis_idx(axis, k - 1, j)] != Rock::None)
                        .unwrap_or(stop)
                }
            } else { i };
            tilted[axis_idx(axis, roll_to, j)] = rock;
        }
    }
    tilted
}

fn axis_idx(axis: Axis, parallel: usize, orthogonal: usize) -> (usize, usize) {
    if axis.index() == 0 {
        (parallel, orthogonal)
    } else {
        (orthogonal, parallel)
    }
}

fn north_load(field: &Array2<Rock>) -> u32 {
    field.rows().into_iter()
        .enumerate()
        .map(|(i, row)| {
            let factor = (field.nrows() - i) as u32;
            let weight = row.fold(0, |acc, &rock| acc + (rock == Rock::Round) as u32);
            factor * weight
        })
        .sum()
}


fn part1(input: String) {
    let field = read_input(input);    
    let tilted = tilt(&field, Axis(0), false);
    println!("{}", north_load(&tilted));
}

fn cycle(field: &Array2<Rock>) -> Array2<Rock> {
    let north = tilt(field, Axis(0), false);
    let west = tilt(&north, Axis(1), false);
    let south = tilt(&west, Axis(0), true);
    tilt(&south, Axis(1), true)
}

fn part2(input: String) {
    const CYCLES: u32 = 1000000000;
    let mut states = HashMap::new();
    let mut field = read_input(input);
    let mut repeat = None;
    for i in 0..CYCLES {
        if let Some(n) = states.get(&field) {
            repeat = Some((*n, i));
            break;
        }
        let cycled = cycle(&field);
        states.insert(field, i); 
        field = cycled;
    }
    match repeat {
        None => println!("No cycles detected!"),
        Some((start, end)) => {
            println!("Cycle: {} <-> {}", start, end);
            let n_cycles = (CYCLES - start) / (end - start);
            let finish = CYCLES - start - (n_cycles * (end - start));
            for _ in 0..finish {
                field = cycle(&field);
            }
        },
    }
    println!("{}", north_load(&field));
}

util::aoc_main!("day14.txt");
