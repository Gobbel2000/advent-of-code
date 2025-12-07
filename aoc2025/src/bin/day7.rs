use std::collections::VecDeque;

fn parse_input(input: &str) -> (Vec<Vec<bool>>, (usize, usize)) {
    let splits = input
        .lines()
        .map(|l| l.chars().map(|c| c == '^').collect())
        .collect();
    // Assume start is on first row
    let start = (input.chars().position(|c| c == 'S').unwrap(), 0);
    (splits, start)
}

fn solve(input: String) {
    let (splits, start) = parse_input(&input);
    let mut nsplits = 0u32;
    let mut timelines = 1u64;
    let mut frontier = VecDeque::from([(start, 1)]);
    while let Some((pos, multiplicity)) = frontier.pop_front() {
        let (x, y) = (pos.0, pos.1 + 1);
        if y == splits.len() {
            // Falls out of bottom
            continue;
        }
        if splits[y][x] {
            nsplits += 1;
            timelines += multiplicity;
            if let Some((b, m2)) = frontier.back_mut()
                && *b == (x - 1, y)
            {
                *m2 += multiplicity;
            } else {
                frontier.push_back(((x - 1, y), multiplicity));
            }
            frontier.push_back(((x + 1, y), multiplicity));
        } else if let Some((b, m2)) = frontier.back_mut()
            && *b == (x, y)
        {
            *m2 += multiplicity;
        } else {
            frontier.push_back(((x, y), multiplicity));
        }
    }
    println!("Part 1: {nsplits}");
    println!("Part 2: {timelines}");
}

fn main() -> std::io::Result<()> {
    let (input, _) = util::get_input("day7.txt")?;
    solve(input);
    Ok(())
}
