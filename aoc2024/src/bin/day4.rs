use euclid::{vec2, default::*};

fn count_xmas(grid: &[&[u8]], pos: (usize, usize)) -> u32 {
    if grid[pos.1][pos.0] != b'X' {
        return 0
    }

    let bounds = Rect::new(Point2D::origin(), Size2D::new(grid[0].len() as i32, grid.len() as i32));
    const DIRS: [Vector2D<i32>; 8] = [
        vec2(1, 0), vec2(-1, 0), vec2(0, 1), vec2(0, -1),
        vec2(1, 1), vec2(1, -1), vec2(-1, 1), vec2(-1, -1),
    ];
    let mut count = 0;
    for dir in DIRS {
        let mut cur = Point2D::from(pos).to_i32();
        let mut found = true;
        for letter in [b'M', b'A', b'S'] {
            cur += dir;
            if !bounds.contains(cur) || grid[cur.y as usize][cur.x as usize] != letter {
                found = false;
                break
            }
        }
        if found {
            count += 1;
        }
    }
    count
}

fn part1(input: String) {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();    
    let count = (0..grid.len()).map(|y| {
            (0..grid[y].len()).map(|x| count_xmas(&grid, (x, y))).sum::<u32>()
        })
        .sum::<u32>();
    println!("{count}");
}

fn is_x_mas(grid: &[&[u8]], pos: (usize, usize)) -> bool {
    if grid[pos.1][pos.0] != b'A' {
        return false
    }

    const DIRS: [Vector2D<i32>; 4] = [vec2(1, -1), vec2(1, 1), vec2(-1, 1), vec2(-1, -1)];
    let pos = Point2D::from(pos).to_i32();
    (0..4).any(|d| {
        let m_pos = [pos + DIRS[d], pos + DIRS[(d + 1) % 4]]; // 2 adjacent positions of M
        let s_pos = [pos + DIRS[(d + 2) % 4], pos + DIRS[(d + 3) % 4]]; // others S
        m_pos.iter().all(|p| grid[p.y as usize][p.x as usize] == b'M') &&
        s_pos.iter().all(|p| grid[p.y as usize][p.x as usize] == b'S')
    })
}

fn part2(input: String) {
    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();    
    let count = (1..grid.len() - 1).map(|y| {
            (1..grid[y].len() - 1).filter(|&x| is_x_mas(&grid, (x, y))).count()
        })
        .sum::<usize>();
    println!("{count}");
}

util::aoc_main!();
