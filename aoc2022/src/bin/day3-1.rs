use std::fs;
use std::process::exit;

static INPUT: &str = "input/day3.txt";

fn main() {
    let input: String = fs::read_to_string(INPUT)
        .expect("Couldn't open input file");
 
    let mut priority_sum: u32 = 0;
 
    let mut lines = input.lines();
    let mut run: bool = true;
    loop {
        let line1 = lines.next().unwrap_or_else(|| { run = false; "" });
        let line2 = lines.next().unwrap_or_else(|| { run = false; "" });
        let line3 = lines.next().unwrap_or_else(|| { run = false; "" });
        if !run {
            break;
        }
 
        let common = find_common_char(line1, line2, line3)
            .unwrap_or_else(|| {
                eprintln!("Couldn't find common char");
                exit(2);
            });

        priority_sum += get_priority(common);
    }
    println!("{}", priority_sum);
}

// Return the first character that is contained in all 3 strings
fn find_common_char(comp1: &str, comp2: &str, comp3: &str) -> Option<char> {
    for c1 in comp1.chars() {
        let find1 = comp2.chars().find(|&c| c == c1);
        let find2 = comp3.chars().find(|&c| c == c1);
        if find1.is_some() && find2.is_some() {
            return Some(c1);
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
