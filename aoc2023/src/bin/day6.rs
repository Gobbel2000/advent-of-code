#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn margin(&self) -> u32 {
        // Find critical points by solving a quadratic formula
        // x: Time pushing button
        // y: Distance traveled after pushing button for x amount of time
        // t: Duration of entire race
        // d: Required distance to win
        //
        //   y = x * (t - x)
        // We want to find x where y = d
        //   x * (t - x) = d
        //   x^2 - tx + d = 0
        let (x1, x2) = solve_quadratic(- (self.time as f64), self.distance as f64);
        // Move solutions by a tiny fraction to get integers that are strictly larger/smaller than
        // the exact values.
        const SMALL: f64 = 0.00001;
        ((x2 - SMALL).floor() - (x1 + SMALL).ceil()) as u32 + 1
    }
}

fn solve_quadratic(p: f64, q: f64) -> (f64, f64) {
    let root = ((p / 2.0).powi(2) - q).sqrt();
    (-(p / 2.0) - root,
     -(p / 2.0) + root)
}

fn parse_input_part1(input: String) -> Vec<Race> {
    let mut lines = input.lines();
    let line_time = lines.next().expect("Missing time line");
    assert!(line_time.starts_with("Time:"));
    let times = line_time[5..].split_whitespace()
        .map(|n| n.parse().unwrap());
    let distance_line = lines.next().expect("Missing distance line");
    assert!(distance_line.starts_with("Distance:"));
    let distances = distance_line[9..].split_whitespace()
        .map(|n| n.parse().unwrap());
    times.zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

fn parse_input_part2(input: String) -> Race {
    let no_kerning = input.replace(' ', "");
    let mut lines = no_kerning.lines();
    let line_time = lines.next().expect("Missing time line");
    assert!(line_time.starts_with("Time:"));
    let time = line_time[5..].parse().unwrap();
    let distance_line = lines.next().expect("Missing distance line");
    assert!(distance_line.starts_with("Distance:"));
    let distance = distance_line[9..].parse().unwrap();
    Race { time, distance }
}

fn part1(input: String) {
    let races = parse_input_part1(input);
    let product: u32 = races.iter()
        .map(Race::margin)
        .product();
    println!("{product}");
}

fn part2(input: String) {
    let race = parse_input_part2(input);
    let margin = race.margin();
    println!("{margin}");
}

util::aoc_main!("day6.txt");
