use std::ops::{Range, RangeInclusive};

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
        })
        .collect()
}

#[inline]
fn area(a: (u32, u32), b: (u32, u32)) -> u64 {
    (a.0.abs_diff(b.0) as u64 + 1) * (a.1.abs_diff(b.1) as u64 + 1)
}

fn part1(input: String) {
    let tiles = parse_input(&input);
    let mut largest = 0;
    for t1 in &tiles {
        for t2 in &tiles {
            let a = area(*t1, *t2);
            if a > largest {
                largest = a;
            }
        }
    }
    println!("{largest}");
}

// Returns true only if t is not inside of the rectangle
#[inline]
fn red_allowed(t: (u32, u32), x_range: Range<u32>, y_range: Range<u32>) -> bool {
    !(t.0 > x_range.start && t.0 + 1 < x_range.end && t.1 > y_range.start && t.1 + 1 < y_range.end)
}

fn is_contained(
    a: (u32, u32),
    b: (u32, u32),
    tiles_x: &[(u32, u32)],
    tiles_y: &[(u32, u32)],
    vert_lines: &[(u32, RangeInclusive<u32>)],
    hori_lines: &[(u32, RangeInclusive<u32>)],
) -> bool {
    let x_range = a.0.min(b.0)..(a.0.max(b.0) + 1);
    let y_range = a.1.min(b.1)..(a.1.max(b.1) + 1);

    // Check that no corners (red tiles) are inside the rectangle
    let corners_ok = if x_range.end - x_range.start <= y_range.end - y_range.start {
        // Use tiles_x
        let start = match tiles_x.binary_search(&(x_range.start, 0)) {
            Ok(e) => e,
            Err(e) => e,
        };
        tiles_x
            .iter()
            .skip(start)
            .take_while(|t| t.0 < x_range.end)
            .filter(|&&t| t != a && t != b)
            .all(|t| red_allowed(*t, x_range.clone(), y_range.clone()))
    } else {
        // Use tiles_y
        let start = match tiles_y.binary_search_by_key(&(y_range.start, 0), |(x, y)| (*y, *x)) {
            Ok(e) => e,
            Err(e) => e,
        };
        tiles_y
            .iter()
            .skip(start)
            .take_while(|t| t.1 < y_range.end)
            .filter(|&&t| t != a && t != b)
            .all(|t| red_allowed(*t, x_range.clone(), y_range.clone()))
    };
    if !corners_ok {
        return false;
    }

    // Check that no line intersects the inside of the rectangle
    let start = match vert_lines.binary_search_by_key(&x_range.start, |(x, _)| *x) {
        Ok(e) => e,
        Err(e) => e,
    };
    for (x, line) in vert_lines
        .iter()
        .skip(start)
        .take_while(|(x, _)| *x < x_range.end)
    {
        if x_range.start < *x
            && x_range.end > *x + 1
            && (y_range.start + 1).max(*line.start()) < (y_range.end - 1).min(line.end() + 1)
        {
            return false;
        }
    }
    let start = match hori_lines.binary_search_by_key(&y_range.start, |(y, _)| *y) {
        Ok(e) => e,
        Err(e) => e,
    };
    for (y, line) in hori_lines
        .iter()
        .skip(start)
        .take_while(|(y, _)| *y < y_range.end)
    {
        if y_range.start < *y
            && y_range.end > *y + 1
            && (x_range.start + 1).max(*line.start()) < (x_range.end - 1).min(line.end() + 1)
        {
            return false;
        }
    }
    true
}

fn part2(input: String) {
    let tiles = parse_input(&input);

    let mut vert_lines = Vec::new();
    let mut hori_lines = Vec::new();
    let mut prev = *tiles.last().unwrap();
    for &t in &tiles {
        if t.0 == prev.0 {
            vert_lines.push((t.0, t.1.min(prev.1)..=t.1.max(prev.1)));
        } else {
            debug_assert_eq!(t.1, prev.1);
            hori_lines.push((t.1, t.0.min(prev.0)..=t.0.max(prev.0)));
        }
        prev = t;
    }
    vert_lines.sort_by_key(|(x, _)| *x);
    hori_lines.sort_by_key(|(y, _)| *y);

    let mut tiles_x = tiles.clone();
    tiles_x.sort();
    let mut tiles_y = tiles.clone();
    tiles_y.sort_by_key(|(x, y)| (*y, *x));
    let mut largest = 0;
    for (idx, t1) in tiles.iter().enumerate() {
        for t2 in tiles.iter().take(idx) {
            let a = area(*t1, *t2);
            if a > largest && is_contained(*t1, *t2, &tiles_x, &tiles_y, &vert_lines, &hori_lines) {
                largest = a;
            }
        }
    }
    println!("{largest}");
}

util::aoc_main!();
