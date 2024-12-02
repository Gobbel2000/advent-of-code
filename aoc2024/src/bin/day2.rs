fn parse(input: String) -> Vec<Vec<i32>> {
    input.lines()
        .map(|l| l.split_whitespace().map(|w| w.parse().unwrap()).collect())
        .collect()
}

fn is_safe(report: impl DoubleEndedIterator<Item=i32> + Clone) -> bool {
    let safety = |a: &i32, b: &i32| (1..=3).contains(&(b - a));
    report.clone().is_sorted_by(safety) || report.rev().is_sorted_by(safety)
}

fn part1(input: String) {
    let reports = parse(input);
    let safe = reports.iter().filter(|r| is_safe(r.iter().copied())).count();
    println!("{safe}");
}

fn is_safe2(report: &[i32]) -> bool {
    (0..report.len()).any(|i| {  // Try with each element removed
        is_safe(report.iter().enumerate().filter(|(j, _)| *j != i).map(|(_, n)| *n))
    })
}

fn part2(input: String) {
    let reports = parse(input);
    let safe = reports.iter().filter(|r| is_safe2(r)).count();
    println!("{safe}");
}

util::aoc_main!();
