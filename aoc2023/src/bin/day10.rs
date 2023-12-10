use std::collections::HashSet;
use std::collections::VecDeque;

use ndarray::Array2;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn apply(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Dir::N => (pos.0 - 1, pos.1),
            Dir::E => (pos.0, pos.1 + 1),
            Dir::S => (pos.0 + 1, pos.1),
            Dir::W => (pos.0, pos.1 - 1),
        }
    }

    fn rev(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
enum Pipe {
    #[default]
    Ground,
    Vertical,
    Horizontal,
    L,
    J,
    SW,
    F,
}

impl Pipe {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            '.' => Ok(Pipe::Ground),
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::L),
            'J' => Ok(Pipe::J),
            '7' => Ok(Pipe::SW),
            'F' => Ok(Pipe::F),
            other => Err(format!("Invalid pipe character: {}", other)),
        }
    }

    // From (N, E, S, W) bools
    fn from_dirs(dirs: (bool, bool, bool, bool)) -> Result<Self, String> {
        match dirs {
            (false, false, false, false) => Ok(Pipe::Ground),
            (true, false, true, false) => Ok(Pipe::Vertical),
            (false, true, false, true) => Ok(Pipe::Horizontal),
            (true, true, false, false) => Ok(Pipe::L),
            (true, false, false, true) => Ok(Pipe::J),
            (false, false, true, true) => Ok(Pipe::SW),
            (false, true, true, false) => Ok(Pipe::F),
            _ => Err("Invalid direction combination".to_string()),
        }
    }

    fn north(&self) -> bool {
        matches!(self, Pipe::Vertical | Pipe::L | Pipe::J)
    }

    fn east(&self) -> bool {
        matches!(self, Pipe::Horizontal | Pipe::L | Pipe::F)
    }

    fn south(&self) -> bool {
        matches!(self, Pipe::Vertical | Pipe::F | Pipe::SW)
    }
    
    fn west(&self) -> bool {
        matches!(self, Pipe::Horizontal | Pipe::SW | Pipe::J)
    }

    fn other_dir(&self, dir: Dir) -> Option<Dir> {
        match (self, dir) {
            (Pipe::Vertical, Dir::S) => Some(Dir::N),
            (Pipe::Vertical, Dir::N) => Some(Dir::S),
            (Pipe::Horizontal, Dir::E) => Some(Dir::W),
            (Pipe::Horizontal, Dir::W) => Some(Dir::E),
            (Pipe::L, Dir::E) => Some(Dir::N),
            (Pipe::L, Dir::N) => Some(Dir::E),
            (Pipe::J, Dir::W) => Some(Dir::N),
            (Pipe::J, Dir::N) => Some(Dir::W),
            (Pipe::SW, Dir::W) => Some(Dir::S),
            (Pipe::SW, Dir::S) => Some(Dir::W),
            (Pipe::F, Dir::E) => Some(Dir::S),
            (Pipe::F, Dir::S) => Some(Dir::E),
            _ => None,
        }
    }
}


enum Spot {
    Pipe(Pipe),
    Start,
}

impl Spot {
    fn from_char(c: char) -> Result<Self, String> {
        match c {
            'S' => Ok(Spot::Start),
            _ => Ok(Spot::Pipe(Pipe::from_char(c)?)),
        }
    }

    fn pipe(&self) -> Option<Pipe> {
        match self {
            Spot::Start => None,
            Spot::Pipe(pipe) => Some(*pipe),
        }
    }
}


fn read_input(input: String) -> Array2<Spot> {
    let lines: Vec<&str> = input.lines().collect();
    let n_lines = lines.len();
    let elements: Vec<Spot> = lines.iter()
        .flat_map(|l| l.chars().map(|c| Spot::from_char(c).unwrap()))
        .collect();
    let n = elements.len();
    Array2::from_shape_vec((n_lines, n / n_lines), elements).unwrap()
}

fn replace_start(mut array: Array2<Spot>) -> ((usize, usize), Array2<Pipe>) {
    for (i, row) in array.rows().into_iter().enumerate() {
        for (j, e) in row.iter().enumerate() {
            if let Spot::Start = e {
                let start_pos = (i, j); 
                let dirs = (
                    array.get((i - 1, j)).expect("S is not at an edge").pipe().unwrap().south(),
                    array.get((i, j + 1)).unwrap().pipe().unwrap().west(),
                    array.get((i + 1, j)).unwrap().pipe().unwrap().north(),
                    array.get((i, j - 1)).unwrap().pipe().unwrap().east(),
                );
                let start_pipe = Pipe::from_dirs(dirs).unwrap();
                array[(i, j)] = Spot::Pipe(start_pipe);
                let pipe_array = array.map(|spot| spot.pipe().unwrap());
                return (start_pos, pipe_array);
            }
        }
    }
    unreachable!();
}

fn part1(input: String) {
    let (start, field) = replace_start(read_input(input));
    let mut dist = 0;
    let mut last_dir = match field[start] {
        Pipe::Vertical | Pipe::F => Dir::N,
        Pipe::L => Dir::E,
        _ => Dir::W,
    };
    let mut pos = start;
    loop {
        last_dir = field[pos].other_dir(last_dir.rev()).unwrap();
        pos = last_dir.apply(pos);
        dist += 1;
        if pos == start {
            break;
        }
    }
    println!("{}", dist / 2);
}

fn included_area(
    start: Pos,
    field: &Array2<Pipe>,
    ring: &HashSet<Pos>,
) -> Option<usize> {
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut inside: HashSet<Pos> = HashSet::new();
    let mut queue = VecDeque::from([start]);

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        // At edge of field, we did not start within the pipe ring, abort
        if pos.0 == 0 || pos.1 == 0 || pos.0 == field.ncols() || pos.1 == field.nrows() {
            return None;
        }

        let adjacent = [
            (pos.0 - 1, pos.1 - 1), // NW
            (pos.0 - 1, pos.1), // NE
            (pos.0, pos.1 - 1), // SW
            (pos.0, pos.1), // SE
        ];

        for adj in adjacent {
            if !ring.contains(&adj) {
                inside.insert(adj);
            }
        }

        // Go north
        let north = (pos.0 - 1, pos.1);
        if !(visited.contains(&north) ||
             (ring.contains(&adjacent[0]) && ring.contains(&adjacent[1])
             && field[adjacent[0]].east() && field[adjacent[1]].west())) {
            queue.push_back(north);
            visited.insert(north);
        }
        // Go east
        let east = (pos.0, pos.1 + 1);
        if !(visited.contains(&east) ||
             (ring.contains(&adjacent[1]) && ring.contains(&adjacent[3])
             && field[adjacent[1]].south() && field[adjacent[3]].north())) {
            queue.push_back(east);
            visited.insert(east);
        }
        // Go south
        let south = (pos.0 + 1, pos.1);
        if !(visited.contains(&south) ||
             (ring.contains(&adjacent[2]) && ring.contains(&adjacent[3])
             && field[adjacent[2]].east() && field[adjacent[3]].west())) {
            queue.push_back(south);
            visited.insert(south);
        }
        // Go west
        let west = (pos.0, pos.1 - 1);
        if !(visited.contains(&west) ||
             (ring.contains(&adjacent[0]) && ring.contains(&adjacent[2])
             && field[adjacent[0]].south() && field[adjacent[2]].north())) {
            queue.push_back(west);
            visited.insert(west);
        }
    }

    Some(inside.len())
}

fn part2(input: String) {
    let (start, field) = replace_start(read_input(input));
    let mut last_dir = match field[start] {
        Pipe::Vertical | Pipe::F => Dir::N,
        Pipe::L => Dir::E,
        _ => Dir::W,
    };
    let mut pos = start;
    // All positions of the pipe ring
    let mut ring: HashSet<(usize, usize)> = HashSet::new();
    ring.insert(pos);
    loop {
        last_dir = field[pos].other_dir(last_dir.rev()).unwrap();
        pos = last_dir.apply(pos);
        if pos == start {
            break;
        }
        ring.insert(pos);
    }

    for search_start in [
        start,
        (start.0, start.1 + 1),
        (start.0 + 1, start.1 + 1),
        (start.0 + 1, start.1)
    ] {
        if let Some(area) = included_area(search_start, &field, &ring) {
            println!("{area}");
            break;
        }
    }
}

util::aoc_main!("day10.txt");
