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

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in ab.iter() {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    let mut m = 0;
    for u in 0..n {
        m = max(m, graph[u].len());
    }
    let colors = (1..=m).collect::<Vec<_>>();
    let mut edges = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = vec![false; n];
    let mut parent = vec![None; n];
    queue.push_back(0);
    visited[0] = true;
    while let Some(u) = queue.pop_front() {
        for (&v, &c) in graph[u]
            .iter()
            .filter(|&&v| !visited[v])
            .zip(
                colors
                    .iter()
                    .filter(|&&c| parent[u].is_none() || c != edges[&(u, parent[u].unwrap())]),
            )
            .collect::<Vec<_>>()
            .iter()
        {
            queue.push_back(v);
            visited[v] = true;
            parent[v] = Some(u);
            edges.insert((u, v), c);
            edges.insert((v, u), c);
        }
    }
    println!("{}", m);
    for &(ai, bi) in ab.iter() {
        println!("{}", edges[&(ai, bi)]);
    }
}
