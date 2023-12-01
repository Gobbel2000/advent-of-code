use std::fs;
use std::process::exit;
use std::rc::Rc;
use std::cell::RefCell;

static INPUT: &str = "input/day20.txt";
const KEY: i64 = 811589153;
const MIX_ROUNDS: u32 = 10;

struct Entry {
    n: i64,
    idx: RefCell<usize>,
}

fn main() {
    let input = fs::read_to_string(INPUT)
        .unwrap_or_else(|e| {
            eprintln!("Could not read input file: {e}");
            exit(1);
        });
    let (order, mut array) = parse_input(&input);
    for _ in 0..MIX_ROUNDS {
        move_entries(&order, &mut array);
    }
    let score = get_score(&array);
    println!("{score}");
}

// Return entries in 2 arrays: One for keeping the original order, the other for tracking the
// positions of all entries
fn parse_input(input: &str) -> (Vec<Rc<Entry>>, Vec<Rc<Entry>>) {
    let mut order = Vec::new();
    let mut array = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let n: i64 = line.parse().unwrap_or_else(|e| {
            eprintln!("Error wile parsing: {e}");
            exit(2);
        });
        // Both lists have a reference counter to the same struct
        let entry = Rc::new(Entry {
            n: n * KEY,
            idx: RefCell::new(i)
        });
        order.push(entry.clone());
        array.push(entry);
    }
    (order, array)
}

fn move_entries(order: &Vec<Rc<Entry>>, list: &mut Vec<Rc<Entry>>) {
    let wrap = order.len() as i64 - 1;
    // Iterate in the original order
    for e in order {
        let mut old_i = e.idx.borrow_mut();
        // Calculate new index
        let mut new_i = (*old_i as i64 + e.n) % wrap;
        if new_i < 0 {
            new_i = wrap + new_i
        }
        let new_i = new_i as usize;

        // Change indices of all moved entries
        if new_i < *old_i {
            for i in new_i..*old_i {
                list[i].idx.replace_with(|&mut old| old + 1);
            }
        } else if *old_i < new_i {
            for i in (*old_i+1)..=new_i {
                list[i].idx.replace_with(|&mut old| old - 1);
            }
        }

        // Move the current entry in the list
        let to_move = list.remove(*old_i);
        *old_i = new_i;
        list.insert(new_i as usize, to_move);
    }
}

fn get_score(list: &Vec<Rc<Entry>>) -> i64 {
    let pos0 = list.iter().position(|e| e.n == 0).expect("List should contain a 0");
    let n = list.len();

    list[(pos0 + 1000) % n].n +
    list[(pos0 + 2000) % n].n +
    list[(pos0 + 3000) % n].n
}
