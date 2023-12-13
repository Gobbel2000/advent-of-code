use ndarray::{Array2, Axis, ArrayView1, Zip};

fn parse_input(input: String) -> Vec<Array2<bool>> {
    input.split("\n\n").map(|block| {
        let lines: Vec<_> = block.lines().collect();
        let elements: Vec<_> = lines.iter()
            .flat_map(|l| l.bytes().map(|b| b == b'#'))
            .collect();
        let n_lines = lines.len();
        Array2::from_shape_vec((n_lines, elements.len() / n_lines), elements).unwrap()
    })
    .collect()
}


fn axis_mirror(field: &Array2<bool>, axis: Axis) -> u32 {
    for (i, lane) in field.axis_windows(axis, 2).into_iter().enumerate() {
        if lane.index_axis(axis, 0) == lane.index_axis(axis, 1) {
            let mut mirrored = true;
            for j in 1..=i {
                let end = i + 1 + j;
                if end >= field.len_of(axis) {
                    break;
                }
                let start = i - j;
                if field.index_axis(axis, start) != field.index_axis(axis, end) {
                    mirrored = false;
                    break;
                }
            }
            if mirrored {
                return i as u32 + 1;
            }
        }
    }
    0
}

fn part1(input: String) {
    let fields = parse_input(input);
    let sum: u32 = fields.iter()
        .map(|f| axis_mirror(f, Axis(1)) + 100 * axis_mirror(f, Axis(0)))
        .sum();
    println!("{sum}");
}


// PART 2

// Number of differences in two lanes. If 0 is returned, both are equal.=
// If 1, that could be the smudge.
fn distance(a: ArrayView1<bool>, b: ArrayView1<bool>) -> u32 {
    Zip::from(a).and(b).fold(0, |acc, a, b| {
        acc + (a ^ b) as u32
    })
}

fn mirror_smudge(field: &Array2<bool>, axis: Axis) -> u32 {
    for (i, lane) in field.axis_windows(axis, 2).into_iter().enumerate() {
        let mut smudges = distance(lane.index_axis(axis, 0), lane.index_axis(axis, 1));
        if smudges <= 1 {
            for j in 1..=i {
                let end = i + 1 + j;
                if smudges > 1 || end >= field.len_of(axis) {
                    break;
                }
                let start = i - j;
                smudges += distance(field.index_axis(axis, start), field.index_axis(axis, end));
            }
            if smudges == 1 {
                return i as u32 + 1;
            }
        }
    }
    0
}

fn part2(input: String) {
    let fields = parse_input(input);
    let sum: u32 = fields.iter()
        .map(|f| mirror_smudge(f, Axis(1)) + 100 * mirror_smudge(f, Axis(0)))
        .sum();
    println!("{sum}");
}

util::aoc_main!("day13.txt");
