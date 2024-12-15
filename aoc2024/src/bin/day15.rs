use euclid::{default::*, vec2};

// Common type for both parts. In part 1 all boxes are BoxL.
#[derive(Clone, Copy)]
enum Spot {
    Empty,
    BoxL,
    BoxR,
    Wall,
}

impl From<u8> for Spot {
    fn from(value: u8) -> Self {
        match value {
            b'.' | b'@' => Spot::Empty,
            b'O' => Spot::BoxL,
            b'#' => Spot::Wall,
            other => panic!("Invalid spot: {other}"),
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<Spot>>, Point2D<i32>, Vec<Vector2D<i32>>) {
    let (field_s, moves_s) = input.split_once("\n\n").unwrap();
    let mut field = Vec::new();
    let mut robot = None;
    for (y, l) in field_s.lines().enumerate() {
        let mut row = Vec::new();
        for (x, b) in l.bytes().enumerate() {
            row.push(Spot::from(b));
            if b == b'@' {
                robot = Some(Point2D::new(x, y).to_i32())
            }
        }
        field.push(row);
    }

    let moves = moves_s
        .bytes()
        .filter(|b| *b != b'\n')
        .map(|b| match b {
            b'^' => vec2(0, -1),
            b'>' => vec2(1, 0),
            b'v' => vec2(0, 1),
            b'<' => vec2(-1, 0),
            other => panic!("Invalid move: {other}"),
        })
        .collect();
    (field, robot.unwrap(), moves)
}

fn gps(field: &[Vec<Spot>]) -> u32 {
    let mut sum = 0;
    for (y, row) in field.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if let Spot::BoxL = s {
                sum += x + 100 * y;
            }
        }
    }
    sum as u32
}

fn part1(input: String) {
    let (mut field, mut robot, moves) = parse(&input);
    for m in moves {
        let next = robot + m;
        match field[next.y as usize][next.x as usize] {
            Spot::Empty => robot = next, // Move into space
            Spot::BoxL => {
                let mut search = next + m;
                let can_move = loop {
                    match field[search.y as usize][search.x as usize] {
                        Spot::BoxL => {}
                        Spot::Wall => break false,
                        Spot::Empty => break true,
                        Spot::BoxR => unreachable!(),
                    }
                    search += m;
                };
                if can_move {
                    robot = next;
                    field[next.y as usize][next.x as usize] = Spot::Empty;
                    field[search.y as usize][search.x as usize] = Spot::BoxL;
                }
            }
            Spot::Wall => {} // Cannot move
            Spot::BoxR => unreachable!(),
        }
    }
    println!("{}", gps(&field));
}

// Transform part 1 field to wider part 2 field
fn widen(field: &[Vec<Spot>]) -> Vec<Vec<Spot>> {
    field
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|s| match s {
                    Spot::Empty => [Spot::Empty; 2],
                    Spot::Wall => [Spot::Wall; 2],
                    Spot::BoxL => [Spot::BoxL, Spot::BoxR],
                    Spot::BoxR => unreachable!(),
                })
                .collect()
        })
        .collect()
}

// Recursively find out whether or not the robot can move in direction `dir` from `start`.
fn can_move_rec(field: &[Vec<Spot>], start: Point2D<i32>, dir: Vector2D<i32>) -> bool {
    let next = start + dir;
    match field[next.y as usize][next.x as usize] {
        Spot::Empty => true,
        Spot::BoxL => can_move_rec(field, next, dir) && can_move_rec(field, next + vec2(1, 0), dir),
        Spot::BoxR => can_move_rec(field, next - vec2(1, 0), dir) && can_move_rec(field, next, dir),
        Spot::Wall => false,
    }
}

// Recursively execute a move for vertical directions into boxes.
fn do_move(field: &mut [Vec<Spot>], start: Point2D<i32>, dir: Vector2D<i32>) {
    let next = start + dir;
    match field[next.y as usize][next.x as usize] {
        Spot::Empty | Spot::Wall => {}
        Spot::BoxL => {
            do_move(field, next, dir);
            do_move(field, next + vec2(1, 0), dir);
            let move_to = next + dir;
            field[next.y as usize][next.x as usize] = Spot::Empty;
            field[next.y as usize][next.x as usize + 1] = Spot::Empty;
            field[move_to.y as usize][move_to.x as usize] = Spot::BoxL;
            field[move_to.y as usize][move_to.x as usize + 1] = Spot::BoxR;
        }
        Spot::BoxR => {
            do_move(field, next - vec2(1, 0), dir);
            do_move(field, next, dir);
            let move_to = next + dir;
            field[next.y as usize][next.x as usize - 1] = Spot::Empty;
            field[next.y as usize][next.x as usize] = Spot::Empty;
            field[move_to.y as usize][move_to.x as usize - 1] = Spot::BoxL;
            field[move_to.y as usize][move_to.x as usize] = Spot::BoxR;
        }
    }
}

fn part2(input: String) {
    let (field1, robot1, moves) = parse(&input);
    let mut field = widen(&field1);
    let mut robot = Point2D::new(robot1.x * 2, robot1.y);
    for m in moves {
        let next = robot + m;
        match field[next.y as usize][next.x as usize] {
            Spot::Empty => robot = next, // Move into space
            Spot::BoxL | Spot::BoxR if m.y == 0 => {
                let mut search = next + m;
                let can_move = loop {
                    match field[search.y as usize][search.x as usize] {
                        Spot::BoxL | Spot::BoxR => {}
                        Spot::Wall => break false,
                        Spot::Empty => break true,
                    }
                    search += m;
                };
                if can_move {
                    robot = next;
                    // Shift boxes by array remove/insert
                    field[next.y as usize].remove(search.x as usize);
                    field[next.y as usize].insert(next.x as usize, Spot::Empty);
                }
            }
            Spot::BoxL | Spot::BoxR => {
                if can_move_rec(&field, robot, m) {
                    do_move(&mut field, robot, m);
                    robot = next;
                }
            }
            Spot::Wall => {} // Cannot move
        }
    }
    println!("{}", gps(&field));
}

util::aoc_main!();
