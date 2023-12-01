use std::fs;
use std::process::exit;
use std::cmp::Ordering;
use std::cell::RefCell;

use fxhash::{FxHashSet, FxHashMap};
use priority_queue::PriorityQueue;

type Point3D = (i8, i8, i8);

static INPUT: &str = "input/day18.txt";
// Some point that lies outside the droplet
const OUTSIDE: Point3D = (0, 0, 0);

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {e}");
            exit(1);
        });
    let cubes = Droplet::from_input(&input);
    let area = cubes.surface_area();
    println!("{area}");
}

struct Droplet {
    cubes: FxHashSet<Point3D>,
    is_outside_cache: RefCell<FxHashMap<Point3D, bool>>,
}

impl Droplet {
    fn from_input(input: &str) -> Self {
        let mut cubes = FxHashSet::default();
        for line in input.lines() {
            let mut coordinates = line.split(',')
                .map(|s| s.parse().expect("Invalid token: {s}"));
            cubes.insert((
                coordinates.next().expect("Missing coordinate x"),
                coordinates.next().expect("Missing coordinate y"),
                coordinates.next().expect("Missing coordinate z")));
        }
        Droplet { cubes, is_outside_cache: RefCell::new(FxHashMap::default()) }
    }

    fn surface_area(&self) -> u32 {
        let mut area = 0;
        for &cube in self.cubes.iter() {
            area += self.surface_of_cube(cube)
        }
        area
    }

    fn surface_of_cube(&self, cube: Point3D) -> u32 {
        let mut surface = 0;
        for adj in self.adjacent_cubes(cube) {
            if !self.cubes.contains(&adj) && self.is_outside(adj) {
                surface += 1
            }
        }
        surface
    }

    fn adjacent_cubes(&self, cube: Point3D) -> Vec<Point3D> {
        [(0, 0, 1), (0, 0, -1), (0, 1, 0), (0, -1, 0), (1, 0, 0), (-1, 0, 0)].iter()
            .map(|dir| (cube.0 + dir.0, cube.1 + dir.1, cube.2 + dir.2))
            .collect()
    }

    // Path finder with a cache
    fn is_outside(&self, point: Point3D) -> bool {
        *self.is_outside_cache.borrow_mut().entry(point)
            .or_insert_with(|| self.has_path(point, OUTSIDE))
    }

    // A* algorithm to find whether a path to a known outside point exists
    fn has_path(&self, start: Point3D, end: Point3D) -> bool {
        let mut distances: FxHashMap<Point3D, u32> = FxHashMap::default();
        distances.insert(start, 0);
        let mut pq = PriorityQueue::new();
        // Reverse sign to get a min-pq
        pq.push(start, OrdF64(dist(start, end) * -1.0));
        while let Some((v, _prio)) = pq.pop() {
            if v == end {
                return true
            }
            for &w in self.adjacent_cubes(v).iter().filter(|c| !self.cubes.contains(c)) {
                if distances.get(&w).unwrap_or(&u32::MAX) > &distances[&v] {
                    distances.insert(w, distances[&v] + 1);
                    pq.push(w, OrdF64((distances[&w] as f64 + dist(w, end)) * -1.0));
                }
            }
        }
        false
    }
}

fn dist(a: Point3D, b: Point3D) -> f64 {
    (((b.0 as i32 - a.0 as i32).pow(2) +
      (b.1 as i32 - a.1 as i32).pow(2) +
      (b.2 as i32 - a.2 as i32).pow(2)) as f64).sqrt()
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
