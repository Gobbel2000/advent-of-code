const N: i32 = 100;

fn parse_line(l: &str) -> (i32, i32) {
    let dir = match l.chars().next().unwrap() {
        'L' => -1,
        'R' => 1,
        _ => panic!(),
    };
    let dist = l[1..].parse::<i32>().unwrap();
    (dir, dist)
}

fn part1(input: String) {
    let mut pos = 50;
    let mut count0 = 0;
    for l in input.lines() {
        let (dir, dist) = parse_line(l);
        pos = (pos + dir * dist) % N;
        if pos == 0 {
            count0 += 1;
        }
    }
    println!("{count0}");
}

fn part2(input: String) {
    let mut pos = 50;
    let mut count0 = 0;
    for l in input.lines() {
        let (dir, dist) = parse_line(l);
        if dir == 1 {
            count0 += (pos + dist) / N;
        } else {
            count0 += ((N - pos) % N + dist) / N;
        }
        pos = (pos + dir * dist).rem_euclid(N);
    }
    println!("{count0}");
}

util::aoc_main!();
