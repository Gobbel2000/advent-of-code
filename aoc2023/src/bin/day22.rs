use euclid::{Point3D, Size3D, vec3, Box3D, UnknownUnit};
use ndarray::{Array2, s};
use rustc_hash::FxHashSet;

type Brick = Box3D<i32, UnknownUnit>;

fn parse_box(line: &str) -> Brick {
    let (min_s, max_s) = line.split_once('~').unwrap();
    let point = |s: &str| {
        let mut nums = s.split(',').map(|n| n.parse().unwrap());
        Point3D::new(
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
        )
    };
    // Switch max point to be exclusive
    Box3D::new(point(min_s), point(max_s) + Size3D::splat(1))
}

fn settle(falling: &mut [Brick]) {
    falling.sort_by_key(|b| b.min.z);
    let footprint = falling.iter()
        .fold(Box3D::zero(), |acc, e| acc.union(e));
    // Initialize at height 1 which is seen as exclusive. The first brick falls to height1.
    let mut highest: Array2<i32> = Array2::ones(footprint.max.xy().to_usize().to_tuple());
    for brick in falling.iter_mut() {
        let mut brick_area = highest.slice_mut(s![brick.to_usize().x_range(), brick.to_usize().y_range()]);
        let fall_height = *brick_area.iter().max().unwrap();
        // No falling up
        assert!(fall_height <= brick.min.z);
        *brick = brick.translate(vec3(0, 0, fall_height - brick.min.z));
        brick_area.fill(brick.max.z);
    }
    falling.sort_by_key(|b| b.min.z);
}

fn get_support_graph(bricks: &[Brick]) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut supports = vec![vec![]; bricks.len()];
    for (i, brick) in bricks.iter().enumerate() {
        // For intersection checking with above bricks
        let grown = brick.inflate(0, 0, 1);
        bricks.iter()
            .enumerate()
            .skip(i + 1)
            .take_while(|(_, b)| b.min.z <= brick.max.z)
            .filter(|(_, b)| b.min.z == brick.max.z && grown.intersects(b))
            .for_each(|(j, _b)| supports[i].push(j));
    }
    let rev: Vec<Vec<usize>> = (0..bricks.len())
        .map(|i| supports.iter()
             .enumerate()
             .filter_map(|(j, adj)| adj.contains(&i).then_some(j))
             .collect()
        )
        .collect();
    (supports, rev)
}

fn removable((supports, rev): (Vec<Vec<usize>>, Vec<Vec<usize>>)) -> u32 {
    let count = supports.iter()
        .filter(|adj| adj.iter().all(|&s| rev[s].len() > 1))
        .count();
    count as u32
}

fn fall_count((supports, rev): (Vec<Vec<usize>>, Vec<Vec<usize>>)) -> u32 {
    let mut count = 0;
    for i in 0..supports.len() {
        let mut level = FxHashSet::default();
        level.insert(i);
        // Collect all fallen bricks
        let mut fallen = level.clone();
        while !level.is_empty() {
            let mut next_lvl = FxHashSet::default();
            for &j in &level {
                for &k in &supports[j] {
                    // j supports k, i supports j via chain
                    if rev[k].iter().all(|e| fallen.contains(e)) {
                        // k is only supported by current level, it will fall
                        next_lvl.insert(k);
                    }
                }
            }
            count += next_lvl.len() as u32;
            fallen.extend(&next_lvl);
            level = next_lvl;
        }
    }
    count
}

fn part1(input: String) {
    let mut bricks: Vec<_> = input.lines().map(parse_box).collect();
    settle(&mut bricks);
    let support_graph = get_support_graph(&bricks);
    println!("{}", removable(support_graph));
}

fn part2(input: String) {
    let mut bricks: Vec<_> = input.lines().map(parse_box).collect();
    settle(&mut bricks);
    let support_graph = get_support_graph(&bricks);
    println!("{}", fall_count(support_graph));
}

util::aoc_main!("day22.txt");
