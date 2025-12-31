use std::collections::HashMap;

// Preassigned indices for special nodes
const YOU: u32 = 0;
const OUT: u32 = 1;
const SVR: u32 = 2;
const DAC: u32 = 3;
const FFT: u32 = 4;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let mut names: HashMap<&str, u32> = HashMap::default();
    // Hardcode indices of special nodes
    names.insert("you", YOU);
    names.insert("out", OUT);
    names.insert("svr", SVR);
    names.insert("dac", DAC);
    names.insert("fft", FFT);
    let mut cur_num: u32 = FFT;
    let mut out = Vec::new();
    for l in input.lines() {
        let (vs, adjs) = l.split_once(": ").unwrap();
        let v = *names.entry(vs).or_insert_with(|| {
            cur_num += 1;
            cur_num
        });
        let mut node = Vec::new();
        let mut max_node = v;
        for adj in adjs.split_ascii_whitespace() {
            let u = *names.entry(adj).or_insert_with(|| {
                cur_num += 1;
                cur_num
            });
            node.push(u);
            max_node = max_node.max(u);
        }
        if max_node as usize >= out.len() {
            out.resize(max_node as usize + 1, Vec::new());
        }
        out[v as usize] = node;
    }
    out
}

type Cache = HashMap<(u32, bool, bool), u64>;

fn dfs(v: u32, graph: &[Vec<u32>], has_dac: bool, has_fft: bool, cache: &mut Cache) -> u64 {
    if v == OUT && has_dac && has_fft {
        return 1;
    }
    if let Some(res) = cache.get(&(v, has_dac, has_fft)) {
        return *res;
    }
    let new_dac = has_dac || v == DAC;
    let new_fft = has_fft || v == FFT;
    let num: u64 = graph[v as usize]
        .iter()
        .map(|&adj| dfs(adj, graph, new_dac, new_fft, cache))
        .sum();
    cache.insert((v, has_dac, has_fft), num);
    num
}

fn part1(input: String) {
    let graph = parse_input(&input);
    println!("{}", dfs(YOU, &graph, true, true, &mut HashMap::default()));
}

fn part2(input: String) {
    let graph = parse_input(&input);
    println!("{}", dfs(SVR, &graph, false, false, &mut HashMap::default()));
}

util::aoc_main!();
