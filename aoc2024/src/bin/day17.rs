use rustc_hash::FxHashMap;

fn parse(input: &str) -> Option<Program> {
    let mut lines = input.lines();
    let a = lines.next()?.split_once(": ")?.1.parse().ok()?;
    let b = lines.next()?.split_once(": ")?.1.parse().ok()?;
    let c = lines.next()?.split_once(": ")?.1.parse().ok()?;
    lines.next()?;
    let program = lines
        .next()?
        .split_once(": ")?
        .1
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<u8>, _>>()
        .ok()?;
    Some(Program {
        a,
        b,
        c,
        out: vec![],
        program,
        ip: 0,
    })
}

#[derive(Debug, Clone, Default)]
struct Program {
    a: u64,
    b: u64,
    c: u64,
    out: Vec<u8>,
    program: Vec<u8>,
    ip: usize,
}

impl Program {
    fn run(&mut self) {
        while self.step() {}
    }

    // Returns true if a step was taken, false if it halted
    fn step(&mut self) -> bool {
        let Some(&[opcode, operand]) = &self.program.get(self.ip..self.ip + 2) else {
            return false;
        };
        self.ip += 2;
        match opcode {
            0 => self.adv(self.combo(operand)),
            1 => self.bxl(operand),
            2 => self.bst(self.combo(operand)),
            3 => self.jnz(operand),
            4 => self.bxc(),
            5 => self.out(self.combo(operand)),
            6 => self.bdv(self.combo(operand)),
            7 => self.cdv(self.combo(operand)),
            _ => panic!(),
        }
        true
    }

    fn combo(&self, operand: u8) -> u64 {
        match operand {
            0..=3 => operand as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn adv(&mut self, x: u64) {
        self.a >>= x
    }

    fn bxl(&mut self, x: u8) {
        self.b ^= x as u64
    }

    fn bst(&mut self, x: u64) {
        self.b = x % 8
    }

    fn jnz(&mut self, x: u8) {
        if self.a != 0 {
            self.ip = x as usize
        }
    }

    fn bxc(&mut self) {
        self.b ^= self.c
    }

    fn out(&mut self, x: u64) {
        self.out.push((x % 8) as u8)
    }

    fn bdv(&mut self, x: u64) {
        self.b = self.a >> x
    }

    fn cdv(&mut self, x: u64) {
        self.c = self.a >> x
    }
}

fn part1(input: String) {
    let mut program = parse(&input).unwrap();
    program.run();
    if let Some(e) = program.out.first() {
        print!("{e}")
    }
    for e in program.out.iter().skip(1) {
        print!(",{e}")
    }
    println!()
}

// Some assumptions on the input:
// * There is exactly one jump instruction at the end of the program, jumping to 0
// * Right before that, an output is generated
// * Right before that, register a is shifted right by 3: adv(3)
//
// Each output depends on at most 10 bits of a (it is used with a shift of at most 7).
// Therefore we look at all 10-bit a's and group them by the first number that is output.
// Then we just need to combine these generators into a chain that fits together.
fn number_generators(mut program: Program) -> [Vec<u16>; 8] {
    let mut out = [const { vec![] }; 8];
    for a in 1..(1 << 10) {
        program.a = a as u64;
        program.out.clear();
        program.ip = 0;
        program.run();
        let &output = program.out.first().unwrap();
        out[output as usize].push(a);
    }
    out
}

fn part2(input: String) {
    let mut program = parse(&input).unwrap();
    let generators = number_generators(program.clone());

    let output = program.program.clone();
    // a_candidates maps from 7-bit required prefixes to the lower bits of a that
    // generate the required numbers so far.
    let mut a_candidates: FxHashMap<u8, u64> = generators[output[0] as usize]
        .iter()
        .rev() // Collects the values for each prefix
        .map(|&a| ((a >> 3) as u8, a as u64 % 8))
        .collect();
    let len = output.len();
    for (i, x) in output.iter().enumerate().skip(1) {
        let mut next_candidates = FxHashMap::default();
        for (prefix, val) in generators[*x as usize]
            .iter()
            .filter(|&a| {
                // Take only short candidates in the end to ensure that not too many numbers are generated
                let max_bits = (len - i) * 3;
                (*a as u64) < (1u64 << max_bits)
            })
            // Only use generators that match any required prefix
            .filter(|&a| a_candidates.contains_key(&((a % (1 << 7)) as u8)))
            .map(|&a| {
                let prefix = (a >> 3) as u8;
                let val = a as u64 % 8;
                let prev = a_candidates[&((a % (1 << 7)) as u8)];
                (prefix, (val << (i * 3)) | prev)
            })
        {
            // Only insert first (smallest) encountered value
            next_candidates.entry(prefix).or_insert(val);
        }
        a_candidates = next_candidates;
    }
    println!("{}", a_candidates[&0]);

    // Verify result
    program.a = a_candidates[&0];
    program.run();
    assert_eq!(program.out, program.program);
}

util::aoc_main!();
