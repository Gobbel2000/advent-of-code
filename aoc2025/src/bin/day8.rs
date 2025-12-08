use euclid::default::Point3D;
use euclid::point3;

fn parse_input(input: &str) -> Vec<Point3D<i64>> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(',').map(|p| p.parse::<i64>().unwrap());
            let (x, y, z) = (
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            );
            point3(x, y, z)
        })
        .collect()
}

// Distances between all points. Reflexive and symmetric pairs are skipped,
// so the Vec's have increasing size, starting at 0.
fn dists(points: &[Point3D<i64>]) -> Vec<Vec<i64>> {
    points
        .iter()
        .enumerate()
        .map(|(idx, &p1)| {
            points
                .iter()
                .take(idx)
                .map(|&p2| (p2 - p1).square_length())
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn sorted_distances(dists: &[Vec<i64>]) -> Vec<(usize, usize, i64)> {
    let mut sorted: Vec<(usize, usize, i64)> = dists
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, d)| (i, j, *d)))
        .collect();
    sorted.sort_by_key(|(_, _, d)| *d);
    sorted
}

fn part1(input: String) {
    let points = parse_input(&input);
    let d = dists(&points);
    let sorted = sorted_distances(&d);

    let mut circuits: Vec<u32> = (0..points.len() as u32).collect();
    for (i, j, _) in sorted.into_iter().take(1000) {
        let new_circuit = circuits[i];
        let old_circuit = circuits[j];
        if new_circuit != old_circuit {
            for c in circuits.iter_mut() {
                if *c == old_circuit {
                    *c = new_circuit;
                }
            }
        }
    }
    let mut sizes: Vec<u32> = vec![0; points.len()];
    for c in circuits {
        sizes[c as usize] += 1
    }
    sizes.sort_unstable();
    let result = sizes.iter().rev().take(3).product::<u32>();
    println!("{result}");
}

fn part2(input: String) {
    let points = parse_input(&input);
    let d = dists(&points);
    let sorted = sorted_distances(&d);

    let mut circuits: Vec<u32> = (0..points.len() as u32).collect();
    for (i, j, _) in sorted.into_iter() {
        let new_circuit = circuits[i];
        let old_circuit = circuits[j];
        if new_circuit != old_circuit {
            let mut all_connected = true;
            for c in circuits.iter_mut() {
                if *c == old_circuit {
                    *c = new_circuit;
                }
                if *c != new_circuit {
                    all_connected = false;
                }
            }
            if all_connected {
                let result = points[i].x * points[j].x;
                println!("{result}");
                return;
            }
        }
    }
}

util::aoc_main!();
