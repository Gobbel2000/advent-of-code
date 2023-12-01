use std::fs;
use std::process::exit;
use std::collections::HashSet;

static INPUT: &str = "input/day9.txt";

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {}", e);
            exit(1);
        });
    
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);
    positions.insert(tail);

    for line in input.lines() {
        let (direction, amount) = parse_line(line).unwrap_or_else(|| {
            eprintln!("Malformed input");
            exit(2);
        });
        for _ in 0..amount {
            move_head(&mut head, direction);
            move_tail(&mut tail, head);
            positions.insert(tail);
        }
    }
    println!("{}", positions.len());
}

fn parse_line(line: &str) -> Option<((i32, i32), u32)> {
    let direction = match line.chars().next()? {
        'R' => (1, 0),
        'D' => (0, 1),
        'L' => (-1, 0),
        'U' => (0, -1),
        _ => return None,
    };
    let amount = line.get(2..)?.parse().ok()?;
    Some((direction, amount))
}

fn move_head(head: &mut (i32, i32), direction: (i32, i32)) {
    *head = tuple_add(*head, direction);
}

fn move_tail(tail: &mut (i32, i32), head: (i32, i32)) {
    if !is_adjacent(*tail, head) {
        *tail = tuple_add(*tail, (
            // Move at most one step in the direction of the head,
            // given by the sign of the difference
            (head.0 - tail.0).signum(),
            (head.1 - tail.1).signum()
        ));
    }
}

fn is_adjacent(tail: (i32, i32), head: (i32, i32)) -> bool {
    (tail.0.abs_diff(head.0) <= 1) &&
    (tail.1.abs_diff(head.1) <= 1)
}

fn tuple_add<T>(a: (T, T), b: (T, T)) -> (T, T)
where T: std::ops::Add<Output = T> {
    (a.0 + b.0, a.1 + b.1)
}
