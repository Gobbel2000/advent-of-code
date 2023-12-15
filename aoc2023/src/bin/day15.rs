use std::mem;

fn hash(s: &str) -> u8 {
    let mut acc: u8 = 0;
    for b in s.bytes() {
        acc = acc.wrapping_add(b);
        acc = acc.wrapping_mul(17);
    }
    acc
}

fn part1(input: String) {
    let hash_sum: u32 = input.split([',', '\n'])
        .map(|s| hash(s) as u32).sum();
    println!("{hash_sum}");
}

struct HashMap {
    table: [Vec<HashEntry>; 256],
}

impl HashMap {
    fn new() -> Self {
        // Some dark magic shenanigans to initialize an array of empty Vec's
        let mut table: [mem::MaybeUninit<Vec<HashEntry>>; 256] = unsafe {
            mem::MaybeUninit::uninit().assume_init()
        };
        for b in &mut table[..] {
            b.write(Vec::new());
        }
        let table = unsafe { mem::transmute::<_, [Vec<HashEntry>; 256]>(table) };
        HashMap {
            table,
        }
    }

    fn insert(&mut self, key: String, val: u8) {
        let h = hash(&key);
        let bucket = &mut self.table[h as usize];
        if let Some(entry) = bucket.iter_mut().find(|entry| entry.label == key) {
            entry.num = val;
        } else {
            bucket.push(HashEntry {
                label: key,
                num: val,
            });
        }
    }

    fn remove(&mut self, key: &str) {
        let h = hash(key);
        let bucket = &mut self.table[h as usize];
        if let Some(idx) = bucket.iter().position(|entry| entry.label == key) {
            bucket.remove(idx);
        }
    }

    fn power(&self) -> u32 {
        self.table.iter().enumerate()
            .map(|(i, bucket)|
                bucket.iter().enumerate()
                    .map(|(j, entry)| (i + 1) * (j + 1) * entry.num as usize)
                    .sum::<usize>()
            )
            .sum::<usize>()
            as u32
    }
}

struct HashEntry {
    label: String,
    num: u8,
}

fn part2(input: String) {
    let mut hashmap = HashMap::new();
    for step in input.split([',', '\n']) {
        if step.is_empty() { continue }
        if let Some((label, num)) = step.split_once('=') {
            hashmap.insert(label.to_string(), num.parse().unwrap());
        } else {
            assert!(step.ends_with('-'));
            hashmap.remove(&step[..step.len() - 1]);
        }
    }
    println!("{}", hashmap.power());
}

util::aoc_main!("day15.txt");
