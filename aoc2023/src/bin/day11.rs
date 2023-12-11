use ndarray::Array2;

fn read_input(input: String) -> Array2<bool> {
    let lines: Vec<&str> = input.lines().collect();
    let n_lines = lines.len();
    let elements: Vec<bool> = lines.iter()
        .flat_map(|l| l.bytes().map(|b| b == b'#'))
        .collect();
    let n = elements.len();
    Array2::from_shape_vec((n_lines, n / n_lines), elements).unwrap()
}

// 2 Arrays indicating for each row (or column) whether it is fully empty (no galaxies)
fn empty_lines(sky: &Array2<bool>) -> (Vec<u32>, Vec<u32>) {
    let row_empty = sky.rows().into_iter().map(|r| !r.iter().any(|e| *e) as u32).collect();
    let col_empty = sky.columns().into_iter().map(|c| !c.iter().any(|e| *e) as u32).collect();
    (row_empty, col_empty)
}

// List of positions of all galaxies
fn galaxies(sky: &Array2<bool>) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    for (i, row) in sky.rows().into_iter().enumerate() {
        for (j, e) in row.iter().enumerate() {
            if *e {
                galaxies.push((i, j));
            }
        }
    }
    galaxies
}

fn solve(input: String, expansion: u64) {
    let sky = read_input(input); 
    let (row_empty, col_empty) = empty_lines(&sky);
    let galaxies = galaxies(&sky);
    // Width one is already present in the field
    let expand_add = expansion - 1;
    let mut sum = 0;
    for (i, g1) in galaxies.iter().enumerate() {
        for g2 in &galaxies[i + 1 ..] {
            let low = (g1.0.min(g2.0), g1.1.min(g2.1));
            let high = (g1.0.max(g2.0), g1.1.max(g2.1));
            // Count number of empty rows in range and multiply by expansion factor
            let vertical_expand = row_empty[low.0..=high.0].iter().sum::<u32>() as u64
                * expand_add;
            let horizontal_expand = col_empty[low.1..=high.1].iter().sum::<u32>() as u64
                * expand_add;
            let distance = ((high.0 - low.0) as u64 + vertical_expand) + 
                           ((high.1 - low.1) as u64 + horizontal_expand);
            sum += distance;
        }
    }
    println!("{sum}");
}

fn part1(input: String) {
    solve(input, 2);
}

fn part2(input: String) {
    solve(input, 1000000);
}

util::aoc_main!("day11.txt");
