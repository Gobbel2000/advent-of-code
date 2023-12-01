use std::fs;
use std::fmt;
use std::process::exit;

static INPUT: &str = "input/day20.txt";

struct Entry {
    n: i32,
    moved: bool,
}

impl From<i32> for Entry {
    fn from(n: i32) -> Self {
        Self { n, moved: false }
    }
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indicator = if self.moved { ' ' } else { 'x' };
        write!(f, "{}{}", self.n, indicator)
    }
}

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {e}");
            exit(1);
        });
    let mut numbers: Vec<Entry> = input.lines().map(|l| Entry::from(l.parse::<i32>()
        .unwrap_or_else(|e| {
            eprintln!("Error wile parsing: {e}");
            exit(2);
        }))).collect();
    move_entries(&mut numbers);
    let score = get_score(&numbers);
    println!("{score}");
}

fn move_entries(list: &mut Vec<Entry>) {
    let mut i = 0;
    let n = list.len();
    while i < n {
        if list[i].moved {
            i += 1;
            continue
        }
        // Wrap at n - 1 because magic
        let wrap = n as i32 - 1;
        let mut new_i = (i as i32 + list[i].n) % wrap;
        if new_i < 0 {
            new_i = wrap + new_i
        }
        let mut to_move = list.remove(i);
        to_move.moved = true;
        list.insert(new_i as usize, to_move);
    }
}

fn get_score(list: &Vec<Entry>) -> i32 {
    let pos0 = list.iter().position(|e| e.n == 0).expect("List should contain a 0");
    let n = list.len();

    list[(pos0 + 1000) % n].n +
    list[(pos0 + 2000) % n].n +
    list[(pos0 + 3000) % n].n
}
