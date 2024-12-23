use rustc_hash::{FxHashMap, FxHashSet};

fn parse(input: &str) -> (Vec<Vec<usize>>, FxHashMap<&str, usize>) {
    let mut graph = Vec::new();
    let mut names: FxHashMap<&str, usize> = FxHashMap::default();
    for l in input.lines() {
        let (vs, ws) = l.split_once('-').unwrap();
        let v = *names.entry(vs).or_insert_with(|| {
            graph.push(vec![]);
            graph.len() - 1
        });
        let w = *names.entry(ws).or_insert_with(|| {
            graph.push(vec![]);
            graph.len() - 1
        });
        graph[v].push(w);
        graph[w].push(v);
    }
    (graph, names)
}

fn part1(input: String) {
    let (graph, names) = parse(&input);
    let mut triples: FxHashSet<[usize; 3]> = FxHashSet::default();
    for (_, &v) in names.iter().filter(|(name, _)| name.starts_with('t')) {
        for (i, &u) in graph[v].iter().enumerate().skip(1) {
            for w in graph[v].iter().take(i) {
                if graph[u].contains(w) {
                    let mut triple = [u, v, *w];
                    triple.sort();
                    triples.insert(triple);
                }
            }
        }
    }
    println!("{}", triples.len());
}

// Bron-Kerbosch algorithm for finding all maximal cliques in a graph
fn bron_kerbosch(
    graph: &[Vec<usize>],
    r: &mut Vec<usize>,
    mut p: FxHashSet<usize>,
    mut x: FxHashSet<usize>,
) -> Vec<Vec<usize>> {
    if p.is_empty() && x.is_empty() {
        return vec![r.to_vec()];
    }
    let mut maximal_cliques = Vec::new();
    let Some(&u) = p.iter().next() else {
        return maximal_cliques;
    };
    let mut p_pivot = p.clone();
    for w in &graph[u] {
        p_pivot.remove(w);
    }
    for v in p_pivot {
        let pn = graph[v].iter().filter(|w| p.contains(w)).copied().collect();
        let xn = graph[v].iter().filter(|w| x.contains(w)).copied().collect();
        r.push(v);
        let new_cliques = bron_kerbosch(graph, r, pn, xn);
        r.pop();
        maximal_cliques.extend(new_cliques);
        p.remove(&v);
        x.insert(v);
    }
    maximal_cliques
}

fn part2(input: String) {
    let (graph, names) = parse(&input);
    let p = (0..graph.len()).collect();
    let mut r = Vec::new();
    let maximal_cliques = bron_kerbosch(&graph, &mut r, p, FxHashSet::default());
    let maximum_clique = maximal_cliques
        .iter()
        .max_by_key(|clique| clique.len())
        .unwrap();
    let mut lan_names: Vec<&str> = names
        .iter()
        .filter(|(_, v)| maximum_clique.contains(v))
        .map(|(name, _)| *name)
        .collect();
    lan_names.sort_unstable();
    println!("{}", lan_names.join(","));
}

util::aoc_main!();
