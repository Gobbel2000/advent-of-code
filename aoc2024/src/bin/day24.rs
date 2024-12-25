use regex::Regex;
use rustc_hash::FxHashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Gate {
    And(usize, usize),
    Or(usize, usize),
    Xor(usize, usize),
}

impl Gate {
    fn and(a: usize, b: usize) -> Self {
        Gate::And(a.min(b), a.max(b))
    }

    fn or(a: usize, b: usize) -> Self {
        Gate::Or(a.min(b), a.max(b))
    }

    fn xor(a: usize, b: usize) -> Self {
        Gate::Xor(a.min(b), a.max(b))
    }

    fn operate(&self, a: bool, b: bool) -> bool {
        match self {
            Gate::And(..) => a & b,
            Gate::Or(..) => a | b,
            Gate::Xor(..) => a ^ b,
        }
    }

    fn deps(&self) -> (usize, usize) {
        match self {
            Gate::And(a, b) => (*a, *b),
            Gate::Or(a, b) => (*a, *b),
            Gate::Xor(a, b) => (*a, *b),
        }
    }
}

#[allow(clippy::type_complexity)]
fn parse(input: &str) -> (Vec<Option<bool>>, Vec<Option<Gate>>, FxHashMap<&str, usize>) {
    let (init_s, gates_s) = input.split_once("\n\n").unwrap();
    let mut names = FxHashMap::default();
    let mut state: Vec<Option<bool>> = init_s
        .lines()
        .map(|l| {
            let (wire, bit) = l.split_once(": ").unwrap();
            let n = names.len();
            names.insert(wire, n);
            Some(bit == "1")
        })
        .collect();
    let re = Regex::new(r"^(\w+) (OR|AND|XOR) (\w+) -> (\w+)$").unwrap();
    let mut gates = vec![None; names.len()];
    for l in gates_s.lines() {
        let (_, [a, op, b, out]) = re.captures(l).unwrap().extract();
        let len = names.len();
        let na = *names.entry(a).or_insert(len);
        let len = names.len();
        let nb = *names.entry(b).or_insert(len);
        let len = names.len();
        let nout = *names.entry(out).or_insert(len);

        let gate = match op {
            "AND" => Gate::and(na, nb),
            "OR" => Gate::or(na, nb),
            "XOR" => Gate::xor(na, nb),
            _ => unreachable!("Constrained by regex"),
        };
        let max_idx = na.max(nb).max(nout);
        if max_idx >= gates.len() {
            gates.resize(max_idx + 1, None);
        }
        gates[nout] = Some(gate);
    }
    state.resize(gates.len(), None);

    (state, gates, names)
}

fn resolve(state: &mut [Option<bool>], gates: &Vec<Option<Gate>>, wire: usize) -> bool {
    if let Some(out) = state[wire] {
        out
    } else {
        let gate = &gates[wire].unwrap();
        let (a, b) = gate.deps();
        let out = gate.operate(resolve(state, gates, a), resolve(state, gates, b));
        state[wire] = Some(out);
        out
    }
}

fn part1(input: String) {
    let (mut state, gates, names) = parse(&input);
    // Verify that 64 bits is enough
    assert!(!names.contains_key("z064"));
    let mut num = 0u64;
    for i in 0..64 {
        let name = format!("z{:02}", i);
        if let Some(idx) = names.get(name.as_str()) {
            let bit = resolve(&mut state, &gates, *idx);
            num |= (bit as u64) << i;
        } else {
            break;
        }
    }
    println!("{num}");
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Name {
    X(u32),
    Y(u32),
    Z(u32),
    Intermediate,
}

impl Name {
    fn num(&self) -> Option<u32> {
        match self {
            Name::X(n) => Some(*n),
            Name::Y(n) => Some(*n),
            Name::Z(n) => Some(*n),
            Name::Intermediate => None,
        }
    }
}

fn name_types(names: &FxHashMap<&str, usize>) -> Vec<Name> {
    let re = Regex::new(r"([xyz])(\d\d)").unwrap();
    let mut name_types = vec![Name::Intermediate; names.len()];
    for (name, &idx) in names.iter() {
        if let Some(cap) = re.captures(name) {
            let (_, [var, n]) = cap.extract();
            let num: u32 = n.parse().unwrap();
            match var {
                "x" => name_types[idx] = Name::X(num),
                "y" => name_types[idx] = Name::Y(num),
                "z" => name_types[idx] = Name::Z(num),
                _ => unreachable!(),
            }
        }
    }
    name_types
}

fn pair_distance(
    (xor_out, carry_out1): (usize, usize),
    (carry_in0, carry_in, z_out, carry_out0): (usize, usize, usize, usize),
    names: &[Name],
    gate: &[Option<Gate>],
    gate_inv: &FxHashMap<Gate, usize>,
) -> u32 {
    let mut dist = 0;
    // Direct connection between xor gates missing
    if !(carry_in0 == xor_out || carry_in == xor_out) {
        dist += 1;
    }

    // Input and output bits don't match
    let in_bit = names[gate[xor_out].unwrap().deps().0].num().unwrap();
    if let Some(out_bit) = names[z_out].num() {
        if in_bit != out_bit {
            dist += 1;
        }
    } else {
        dist += 1;
    }

    // There is no carry or gate using both carry outputs
    let or_gate = Gate::or(carry_out0, carry_out1);
    if !gate_inv.contains_key(&or_gate) {
        dist += 1;
    }

    dist
}

fn part2(input: String) {
    let (state, gates, names) = parse(&input);
    let gate_inv: FxHashMap<Gate, usize> = gates
        .iter()
        .enumerate()
        .filter_map(|(i, gate)| gate.map(|g| (g, i)))
        .collect();
    // Bit 0 is ignored, checked manually
    let input_pairs: Vec<(usize, usize)> = (1..64)
        .map_while(|i| {
            let xname = format!("x{:02}", i);
            let yname = format!("y{:02}", i);
            let &xid = names.get(xname.as_str())?;
            let &yid = names.get(yname.as_str())?;
            let xor_gate = Gate::Xor(xid, yid);
            let and_gate = Gate::And(xid, yid);
            Some((gate_inv[&xor_gate], gate_inv[&and_gate]))
        })
        .collect();
    let mut carry_xors = Vec::new();
    let mut carry_ands = Vec::new();
    for (i, opt) in gates.iter().enumerate() {
        let Some(g) = opt else { continue };
        let (a, b) = g.deps();
        if state[a].is_none() && state[b].is_none() {
            match g {
                Gate::Xor(..) => carry_xors.push((g, i)),
                Gate::And(..) => carry_ands.push((g, i)),
                Gate::Or(..) => {}
            }
        }
    }
    let carry_pairs: Vec<(usize, usize, usize, usize)> = carry_xors
        .iter()
        .filter_map(|(gate, out)| {
            let (a, b) = gate.deps();
            let (_, and_out) = carry_ands
                .iter()
                .find(|(and_gate, _out)| and_gate.deps() == (a, b))?;
            Some((a, b, *out, *and_out))
        })
        .collect();

    let mut names_rev = vec![""; names.len()];
    for (name, i) in names.iter() {
        names_rev[*i] = name;
    }
    let name_t = name_types(&names);
    let mut carry_order = Vec::with_capacity(carry_pairs.len());
    for input_pair in input_pairs.iter() {
        let (dist, min_pair) = carry_pairs
            .iter()
            .map(|carry_pair| {
                let dist = pair_distance(*input_pair, *carry_pair, &name_t, &gates, &gate_inv);
                (dist, carry_pair)
            })
            .min()
            .unwrap();
        carry_order.push((dist, min_pair));
    }

    let mut wrong_outputs = Vec::with_capacity(8);
    for (i, (input_pair, &(dist, pair))) in input_pairs.iter().zip(&carry_order).enumerate() {
        if dist > 0 {
            println!("\nBit: {}", i + 1);
            let (xor_out, carry_out1) = *input_pair;
            let (carry_in0, carry_in, z_out, carry_out0) = *pair;
            // Direct connection between xor gates missing
            if !(carry_in0 == xor_out || carry_in == xor_out) {
                println!("1 {}", names_rev[xor_out]);
                wrong_outputs.push(names_rev[xor_out]);
            }

            // Input and output bits don't match
            let in_bit = name_t[gates[xor_out].unwrap().deps().0].num().unwrap();
            if let Some(out_bit) = name_t[z_out].num() {
                if in_bit != out_bit {
                    println!("2 {} should be z{}", names_rev[z_out], i + 1);
                    wrong_outputs.push(names_rev[z_out]);
                }
            } else {
                println!("3 {} should be z{}", names_rev[z_out], i + 1);
                wrong_outputs.push(names_rev[z_out]);
            }

            let prev_input_pair = input_pairs[i - 1];
            let (_, prev_carry_pair) = carry_order[i - 1];
            let prev_c_gate = Gate::or(prev_input_pair.1, prev_carry_pair.3);
            let prev_c = gate_inv[&prev_c_gate];

            let &(a, b, ..) = carry_order[i + 1].1;
            let (c_a, _c_b) = gates[a].unwrap().deps();
            let next_c = if name_t[c_a] == Name::X(i as u32 + 2) {
                b
            } else {
                a
            };
            let next_c_gate = gates[next_c].unwrap();
            let (a, b) = next_c_gate.deps();
            if carry_out0 != a && carry_out1 != a {
                if carry_out0 == b {
                    println!("40 {}", names_rev[carry_out1]);
                    wrong_outputs.push(names_rev[carry_out1]);
                } else if carry_out1 == b {
                    println!("41 {}", names_rev[carry_out0]);
                    wrong_outputs.push(names_rev[carry_out0]);
                } else {
                    let or_gate = Gate::or(carry_out0, carry_out1);
                    if let Some(&or_out) = gate_inv.get(&or_gate) {
                        println!("45 {}", names_rev[or_out]);
                        wrong_outputs.push(names_rev[or_out]);
                    }
                }
            }
            if carry_out0 != b && carry_out1 != b {
                if carry_out0 == a {
                    println!("42 {}", names_rev[carry_out1]);
                    wrong_outputs.push(names_rev[carry_out1]);
                }
                if carry_out1 == a {
                    println!("43 {}", names_rev[carry_out0]);
                    wrong_outputs.push(names_rev[carry_out0]);
                }
            }

            if prev_c != carry_in && prev_c != carry_in0 {
                println!("6 {}", names_rev[prev_c]);
                wrong_outputs.push(names_rev[prev_c]);
            }

            println!("Previous C: {}", names_rev[prev_c]);
            println!("Next C: {}", names_rev[next_c]);
        }
    }
    wrong_outputs.sort();
    println!();
    println!("{}", wrong_outputs.join(","));
}

util::aoc_main!();
