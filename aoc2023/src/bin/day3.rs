use std::collections::HashMap;

use regex::Regex;

#[inline]
fn part_symbol(chr: u8) -> bool {
    (chr != b'.') && !chr.is_ascii_digit()
}

fn part1(input: String) {
    let re = Regex::new(r"\d+").unwrap();
    let lines: Vec<&str> = input.lines().collect();    
    let height = lines.len();
    let width = lines[0].len();  // Assume all lines have equal length

    let mut sum = 0;
    for (row, line) in lines.iter().enumerate() {
        for m in re.find_iter(line) {
            let col_start = m.start().max(1) - 1;
            let col_end = m.end().min(width - 1); // Inclusive
            
            if (row >= 1 && 
                    (col_start..=col_end).any(|c| part_symbol(lines[row - 1].as_bytes()[c])))
                || ((row + 1) < height && 
                    (col_start..=col_end).any(|c| part_symbol(lines[row + 1].as_bytes()[c])))
                || part_symbol(lines[row].as_bytes()[col_start])
                || part_symbol(lines[row].as_bytes()[col_end])
            {
                sum += m.as_str().parse::<u32>().unwrap();
            }
        }
    }
    println!("{sum}");
}

fn part2(input: String) {
    let re = Regex::new(r"\d+").unwrap();
    let lines: Vec<&str> = input.lines().collect();    
    let height = lines.len();
    let width = lines[0].len();  // Assume all lines have equal length

    // Maps Gear position => adjacent part numbers
    let mut gear_table: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for m in re.find_iter(line) {
            let col_start = m.start().max(1) - 1;
            let col_end = (m.end() + 1).min(width); // Exclusive
            let row_start = row.max(1) - 1;
            let row_end = (row + 2).min(height); // Also exclusive

            for (y, cur_line) in lines.iter().enumerate().take(row_end).skip(row_start) {
                for (x, byte) in cur_line.bytes().enumerate().take(col_end).skip(col_start) {
                    if byte == b'*' {
                        let num = m.as_str().parse::<u32>().unwrap();
                        gear_table.entry((x, y)).or_default().push(num);
                    }
                }
            }
        }
    }
    let sum: u32 = gear_table.values()
        .filter(|vec| vec.len() == 2)  // Gears have exactly two adjacent part numbers
        .map(|vec| vec[0] * vec[1])  // Multiply part numbers for gear "ratio"
        .sum();
    println!("{sum}");
}

util::aoc_main!("day3.txt");
