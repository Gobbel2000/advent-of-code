use std::fs;
use std::process::exit;

use fxhash::FxHashSet;

static INPUT: &str = "input/day18.txt";

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {e}");
            exit(1);
        });
    let cubes = parse_input(&input);
    let area = surface_area(cubes);
    println!("{area}");
}

fn surface_area(cubes: FxHashSet<(i8, i8, i8)>) -> u32 {
    let mut area = 0;
    for cube in &cubes {
        area += surface_of_cube(&cubes, *cube)
    }
    area
}

fn surface_of_cube(cubes: &FxHashSet<(i8, i8, i8)>, cube: (i8, i8, i8)) -> u32 {
    let mut surface = 0;
    for dir in [(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)] {
        let other = (cube.0 + dir.0, cube.1 + dir.1, cube.2 + dir.2);
        if !cubes.contains(&other) {
            surface += 1
        }
    }
    surface
}

fn parse_input(input: &str) -> FxHashSet<(i8, i8, i8)> {
    let mut cubes = FxHashSet::default();
    for line in input.lines() {
        let mut coordinates = line.split(',')
            .map(|s| s.parse().expect("Invalid token: {s}"));
        cubes.insert((
            coordinates.next().expect("Missing coordinate x"),
            coordinates.next().expect("Missing coordinate y"),
            coordinates.next().expect("Missing coordinate z")));
    }
    cubes
}
