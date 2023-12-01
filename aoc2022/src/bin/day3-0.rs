use std::fs;
use std::process::exit;

static INPUT: &str = "input/day3.txt";

fn main() {
    let input: String = fs::read_to_string(INPUT)
        .expect("Couldn't open input file");
 
    let mut priority_sum: u32 = 0;
 
    for line in input.lines() {
        let n_items = line.len();
        if n_items % 2 != 0 {
            eprintln!("Not an even amount of items in Rucksack");
            exit(1);
        }
        let n_compare = n_items / 2;
 
        let comp1 = &line[..n_compare];
        let comp2 = &line[n_compare..];
 
        let common = find_common_char(comp1, comp2)
            .unwrap_or_else(|| {
                eprintln!("Couldn't find common char");
                exit(2);
            });

        priority_sum += get_priority(common);
    }
    println!("{}", priority_sum);
}

// Return the first character that is contained in both strings
fn find_common_char(comp1: &str, comp2: &str) -> Option<char> {
    for c1 in comp1.chars() {
        let find = comp2.chars().find(|&c| c == c1);
        if find.is_some() {
            return find;
        }
    }
    None        
}

fn get_priority(item: char) -> u32 {
    let ascii = item as u32;
    match item {
        'a'..='z' => ascii - ('a' as u32 - 1),
        'A'..='Z' => ascii - ('A' as u32 - 27),
        _ => 0,
    }
}
