fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c == '@').collect())
        .collect()
}

fn count_adj(grid: &[Vec<bool>], (x, y): (usize, usize)) -> usize {
    let width = grid[0].len();
    let height = grid.len();
    grid.iter()
        .take((y + 2).min(height))
        .skip(y.saturating_sub(1))
        .map(|r| {
            r.iter()
                .take((x + 2).min(width))
                .skip(x.saturating_sub(1))
                .take(3)
                .filter(|e| **e)
                .count()
        })
        .sum::<usize>()
}

fn part1(input: String) {
    let grid = parse_input(&input);
    let mut count = 0u32;
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate().filter(|(_, r)| **r) {
            let n_adj = count_adj(&grid, (x, y));
            // Center roll is counted too
            if n_adj < 5 {
                count += 1;
            }
        }
    }
    println!("{count}");
}

fn part2(input: String) {
    let mut grid = parse_input(&input);
    let mut removed = 0u32;
    loop {
        let mut next_grid = grid.clone();
        let prev_removed = removed;
        for (y, row) in grid.iter().enumerate() {
            for (x, _) in row.iter().enumerate().filter(|(_, r)| **r) {
                let n_adj = count_adj(&grid, (x, y));
                // Center roll is counted too
                if n_adj < 5 {
                    next_grid[y][x] = false;
                    removed += 1;
                }
            }
        }
        if removed == prev_removed {
            break;
        }
        grid = next_grid;
    }
    println!("{}", removed);
}

util::aoc_main!();
