use ndarray::Array2;
use rustc_hash::FxHashSet;

fn parse_input(input: String) -> (Array2<bool>, (usize, usize)) {
    let lines: Vec<_> = input.lines().collect();
    let n_lines = lines.len();
    let elements: Vec<bool> = lines.iter().flat_map(|l|
            l.bytes().map(|b| {
                b == b'#'
            })
        )
        .collect();
    let mut start = (0, 0);
    for (i, l) in lines.iter().enumerate() {
        if let Some(j) = l.bytes().position(|b| b == b'S') {
            start = (i, j);
            break;
        }
    }
    let array = Array2::from_shape_vec((n_lines, elements.len() / n_lines), elements).unwrap();
    (array, start)
}

fn get_adj(field: &Array2<bool>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut adj = Vec::new();
    if pos.0 > 0 {
        adj.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        adj.push((pos.0, pos.1 - 1));
    }
    if pos.0 + 1 < field.nrows() {
        adj.push((pos.0 + 1, pos.1));
    }
    if pos.1 + 1 < field.ncols() {
        adj.push((pos.0, pos.1 + 1));
    }
    adj.retain(|p| !field[*p]);
    adj
}

fn adj_unbounded(field: &Array2<bool>, pos: (isize, isize)) -> Vec<(isize, isize)> {
    let mut adj = vec![
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 + 1),
    ];
    adj.retain(|p| {
        let bounded_pos = (p.0.rem_euclid(field.nrows() as isize) as usize,
                           p.1.rem_euclid(field.ncols() as isize) as usize);
        !field[bounded_pos]
    });
    adj
}

fn walk(field: &Array2<bool>, start: (usize, usize), steps: u64) -> u64 {
    let mut start_pos: FxHashSet<(usize, usize)> = FxHashSet::default();
    start_pos.insert(start);
    let mut end_pos: FxHashSet<(usize, usize)> = FxHashSet::default();
    
    for _ in 0..steps {
        for pos in &start_pos {
            end_pos.extend(get_adj(field, *pos));
        }
        std::mem::swap(&mut start_pos, &mut end_pos);
    }

    start_pos.len() as u64
}

fn walk_part2(field: &Array2<bool>, start: (usize, usize), steps: Vec<u64>) -> Vec<u64> {
    let mut start_pos: FxHashSet<(isize, isize)> = FxHashSet::default();
    start_pos.insert((start.0 as isize, start.1 as isize));
    let mut end_pos: FxHashSet<(isize, isize)> = FxHashSet::default();
    
    let mut counts = Vec::new();
    for i in 1..=steps.last().copied().unwrap_or_default() {
        for pos in &start_pos {
            end_pos.extend(adj_unbounded(field, *pos));
        }
        std::mem::swap(&mut start_pos, &mut end_pos);
        if steps.contains(&i) {
            counts.push(start_pos.len() as u64);   
        }
    }

    counts
}

fn part1(input: String) {
    const STEPS: u64 = 64;
    let (field, start) = parse_input(input);    
    let positions = walk(&field, start, STEPS);
    println!("{positions}");
}

fn part2(input: String) {
    const STEPS: u64 = 26501365;
    let (field, start) = parse_input(input);    

    // Assume the following:
    // Field is quadratic
    assert_eq!(field.nrows(), field.ncols());
    let n = field.nrows() as u64;
    // Field size is uneven
    assert!(n % 2 == 1);
    // Start is in the middle
    assert_eq!(start, (n as usize / 2, n as usize / 2));

    let steps = vec![n / 2, (n / 2) + n, (n / 2) + 2*n];
    let counts = walk_part2(&field, start, steps);
    // Fit quadratic function to these 3 points
    let y0 = counts[0] as i64;
    let y1 = counts[1] as i64;
    let y2 = counts[2] as i64;
    println!("{counts:?}");
    // Apply hardcoded matric inversion
    let a = y0 / 2 - y1 + y2 / 2;
    let b = - 3 * y0 / 2 + 2 * y1 - y2 / 2;
    let c = y0;
    
    let x = (STEPS / n) as i64;
    // Run quadratic function
    let positions = a * x.pow(2) + b * x + c;
    println!("{positions}");
}

util::aoc_main!("day21.txt");
