use euclid::default::*;

const N_ANTENNAS: usize = (b'z' - b'0') as usize + 1;
// For each frequency (from b'0' to b'z') the list of antenna positions
type Antennas = Box<[Vec<Point2D<i32>>]>;

fn parse(input: String) -> (Antennas, Rect<i32>) {
    let mut antennas = vec![Vec::new(); N_ANTENNAS].into_boxed_slice();
    let mut width = 0;
    let mut height = 0;
    for (y, l) in input.lines().enumerate() {
        height = y + 1;
        if width == 0 {
            width = l.len()
        } else {
            assert!(width == l.len())
        }
        for (x, b) in l.bytes().enumerate().filter(|(_, b)| *b != b'.') {
            antennas[(b - b'0') as usize].push(Point2D::new(x, y).to_i32())
        }
    }
    let bounds = Rect::new(Point2D::origin(), Size2D::new(width, height).to_i32());
    (antennas, bounds)
}

fn part1(input: String) {
    let (antennas, bounds) = parse(input);
    let mut antinodes = vec![vec![false; bounds.width() as usize]; bounds.height() as usize];
    for list in antennas.iter().filter(|l| !l.is_empty()) {
        for (i, &a) in list.iter().enumerate().skip(1) {
            for &b in list.iter().take(i) {
                let diff = b - a;
                let ax = a - diff;
                if bounds.contains(ax) {
                    antinodes[ax.y as usize][ax.x as usize] = true;
                }
                let bx = b + diff;
                if bounds.contains(bx) {
                    antinodes[bx.y as usize][bx.x as usize] = true;
                }
            }
        }
    }
    let sum = antinodes
        .iter()
        .map(|row| row.iter().map(|b| u32::from(*b)).sum::<u32>())
        .sum::<u32>();
    println!("{sum}");
}

fn part2(input: String) {
    let (antennas, bounds) = parse(input);
    let mut antinodes = vec![vec![false; bounds.width() as usize]; bounds.height() as usize];
    for list in antennas.iter().filter(|l| !l.is_empty()) {
        for (i, &a) in list.iter().enumerate().skip(1) {
            for &b in list.iter().take(i) {
                let diff = b - a;
                // Start at antenna a, keep going until hitting bounds
                let mut ax = a;
                while bounds.contains(ax) {
                    antinodes[ax.y as usize][ax.x as usize] = true;
                    ax -= diff;
                }
                let mut bx = b;
                while bounds.contains(bx) {
                    antinodes[bx.y as usize][bx.x as usize] = true;
                    bx += diff;
                }
            }
        }
    }
    let sum = antinodes
        .iter()
        .map(|row| row.iter().map(|b| u32::from(*b)).sum::<u32>())
        .sum::<u32>();
    println!("{sum}");
}

util::aoc_main!();
