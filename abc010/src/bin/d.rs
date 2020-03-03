use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn dfs(
    graph: &Vec<Vec<(usize, isize)>>,
    u: usize,
    t: usize,
    f: isize,
    capacity: &mut HashMap<(usize, usize), isize>,
    visited: &mut HashSet<usize>,
) -> isize {
    if u == t {
        return f;
    }
    visited.insert(u);
    for &(v, _) in &graph[u] {
        let c = capacity[&(u, v)];
        if visited.contains(&v) || c <= 0 {
            continue;
        }
        let d = dfs(graph, v, t, min(f, c), capacity, visited);
        if d > 0 {
            *capacity.get_mut(&(u, v)).unwrap() -= d;
            *capacity.get_mut(&(v, u)).unwrap() += d;
            return d;
        }
    }
    0
}

fn ford_fulerson(graph: &Vec<Vec<(usize, isize)>>, s: usize, t: usize) -> isize {
    let n = graph.len();
    let mut capacity = HashMap::new();
    for u in 0..n {
        for &(v, c) in &graph[u] {
            capacity.insert((u, v), c);
        }
    }
    let mut flow = 0;
    loop {
        let mut visited = HashSet::new();
        let f = dfs(graph, s, t, INF as isize, &mut capacity, &mut visited);
        if f == 0 {
            break;
        }
        flow += f;
    }
    flow
}

fn main() {
    input! {
        n: usize,
        g: usize,
        e: usize,
        p: [usize; g],
        ab: [(usize, usize); e],
    }
    let mut graph = vec![vec![]; n + 2 * e + 1];
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        graph[ai].push((n + i, 1));
        graph[n + i].push((bi, 1));
        graph[bi].push((n + i, 0));
        graph[n + i].push((ai, 0));

        graph[bi].push((n + e + i, 1));
        graph[n + e + i].push((ai, 1));
        graph[ai].push((n + e + i, 0));
        graph[n + e + i].push((bi, 0));
    }
    for &pi in &p {
        graph[pi].push((n + 2 * e, 1));
        graph[n + 2 * e].push((pi, 0));
    }
    println!("{}", ford_fulerson(&graph, 0, n + 2 * e));
}
