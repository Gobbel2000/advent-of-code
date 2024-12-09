fn part1(input: String) {
    let mut id: u64 = 0;
    let mut disk = Vec::new();
    let mut file = true;
    for b in input.trim().bytes() {
        let num: usize = (b - b'0') as usize;
        if file {
            disk.extend(vec![Some(id); num]);
            id += 1;
        } else {
            disk.extend(vec![None; num]);
        }
        file = !file;
    }

    let mut first_free = 0;
    while disk[first_free].is_some() {
        first_free += 1
    }

    let mut last_file = disk.len() - 1;
    while disk[last_file].is_none() {
        last_file -= 1
    }

    while first_free < last_file {
        disk[first_free] = disk[last_file];
        disk[last_file] = None;
        while disk[first_free].is_some() {
            first_free += 1
        }
        while disk[last_file].is_none() {
            last_file -= 1
        }
    }

    let checksum = disk
        .iter()
        .filter_map(|e| *e)
        .enumerate()
        .map(|(i, id)| i as u64 * id)
        .sum::<u64>();
    println!("{checksum}");
}

fn part2(input: String) {
    // Tuples of (idx, size)
    let mut free_spaces = Vec::new();
    // Tuples of (idx, size, id)
    let mut files = Vec::new();

    let mut id: u64 = 0;
    let mut disk_len = 0;
    let mut file = true;
    for b in input.trim().bytes() {
        let num = (b - b'0') as u64;
        if file {
            files.push((disk_len, num, id));
            id += 1;
        } else {
            free_spaces.push((disk_len, num));
        }
        disk_len += num;
        file = !file;
    }

    for (idx, size, _id) in files.iter_mut().rev() {
        match free_spaces
            .iter_mut()
            // Only search spaces left of file
            .take_while(|(sp_idx, _space)| sp_idx < idx)
            .find(|(_sp_idx, space)| space >= size)
        {
            None => {} // No space found
            Some((sp_idx, space)) => {
                // Move file into space
                *idx = *sp_idx;
                // Reduce free space
                *sp_idx += *size;
                *space -= *size;
            }
        }
    }

    let sum_range = |n| if n == 0 { 0 } else { (n * (n - 1)) / 2 };
    let checksum = files
        .iter()
        .map(|(idx, size, id)| (sum_range(idx + size) - sum_range(*idx)) * id)
        .sum::<u64>();
    println!("{checksum}");
}

util::aoc_main!();
