use std::ops::RangeInclusive;
use ndarray::{Array2, Array1};
use ndarray_linalg::solve::Solve;

type Point = [f64; 3];

#[derive(Debug, Clone)]
struct Hail {
    pos: Point,
    vel: Point,
}

impl Hail {
    fn linear_components_xy(&self) -> (f64, f64) {
        let m = self.vel[1] / self.vel[0];
        let n = self.pos[1] - m * self.pos[0];
        (m, n)
    }

    fn intersection_xy(&self, other: &Hail) -> (f64, f64) {
        let (m1, n1) = self.linear_components_xy();
        let (m2, n2) = other.linear_components_xy();
        let x = (n2 - n1) / (m1 - m2);
        let y = x * m1 + n1;
        (x, y)
    }

    fn is_future(&self, x: f64) -> bool {
        x.partial_cmp(&(self.pos[0])) == self.vel[0].partial_cmp(&0.0)
    }
}

fn parse_input(input: String) -> Vec<Hail> {
    input.lines().map(|l| {
        let (pos_s, vel_s) = l.split_once(" @ ").unwrap();
        let mut pos_it = pos_s.splitn(3, ", ").map(|n| n.trim().parse::<f64>().unwrap());
        let pos = [pos_it.next().unwrap(), pos_it.next().unwrap(), pos_it.next().unwrap()];
        let mut vel_it = vel_s.splitn(3, ", ").map(|n| n.trim().parse::<f64>().unwrap());
        let vel = [vel_it.next().unwrap(), vel_it.next().unwrap(), vel_it.next().unwrap()];
        assert!(vel[0] != 0.0);
        Hail { pos, vel }
    })
    .collect()
}

fn part1(input: String) {
    const AREA: RangeInclusive<f64> = 200000000000000.0..=400000000000000.0;
    //const AREA: RangeInclusive<f64> = 7.0..=27.0;
    let hail = parse_input(input);    
    let mut intersections = 0;
    for (i, h1) in hail.iter().enumerate() {
        for h2 in hail.iter().skip(i + 1) {
            let int = h1.intersection_xy(h2);
            if h1.is_future(int.0) && h2.is_future(int.0)
                && AREA.contains(&int.0) && AREA.contains(&int.1)
            {
                intersections += 1;
            }
        }
    }
    println!("{intersections}");
}

fn linear_system2(hail: &[Hail], x: usize, y: usize) -> (Array2<f64>, Array1<f64>) {
    // Return Coefficient matrix and result vector for unknowns:
    // [x, y, dx, dy]
    // of the stone, where x and y can be adjusted via arguments.
    // x=0, y=1 means actual x and y
    // x=1, y=2 means calculate y and z
    let p: Vec<_> = hail.iter().take(5).map(|h| h.pos).collect();
    let d: Vec<_> = hail.iter().take(5).map(|h| h.vel).collect();
    let mut res = Array1::zeros(4);
    let mut coeff = Array2::zeros((4, 4));
    for i in 0..4 {
        let j = i + 1;
        res[i] = (p[j][x] * d[j][y] - p[j][y] * d[j][x]) -
                 (p[i][x] * d[i][y] - p[i][y] * d[i][x]);
        // Xs, Ys
        coeff[(i, 0)] = d[j][y] - d[i][y];
        coeff[(i, 1)] = d[i][x] - d[j][x];
        // dXs, dYs
        coeff[(i, 2)] = p[j][x] - p[i][x];
        coeff[(i, 3)] = p[j][y] - p[i][y];
    }
    (coeff, res)
}

fn part2(input: String) {
    let hail = parse_input(input);
    // Find x and y
    let (axy, bxy) = linear_system2(&hail, 0, 1);
    // Find z (and y again)
    let (ayz, byz) = linear_system2(&hail, 1, 2);
    let xy = axy.solve(&bxy).unwrap();
    let yz = ayz.solve(&byz).unwrap();
    println!("X, Y, dX, dY = \n{:?}\n", xy);
    println!("Y, Z, dY, dZ = \n{:?}\n", yz);
    println!("{}", (xy[0] + xy[1] + yz[1]).round());
}

util::aoc_main!("day24.txt");
