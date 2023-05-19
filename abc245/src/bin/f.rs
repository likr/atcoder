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

fn dfs(
    graph: &Vec<Vec<usize>>,
    u: usize,
    visited: &mut HashSet<usize>,
    result: &mut HashMap<usize, bool>,
) {
    for &v in graph[u].iter() {
        if visited.contains(&v) {
            result.insert(v, true);
        } else if !result.contains_key(&v) {
            visited.insert(v);
            dfs(graph, v, visited, result);
        }
    }
    result.insert(u, graph[u].iter().any(|&v| result[&v]));
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let (ui, vi) = uv[i];
        graph[ui].push(vi);
    }
    let mut result = HashMap::new();
    for u in 0..n {
        if !result.contains_key(&u) {
            let mut visited = HashSet::new();
            dfs(&graph, u, &mut visited, &mut result);
        }
    }
    let mut count = 0;
    for u in 0..n {
        if result[&u] {
            count += 1;
        }
    }
    println!("{}", count);
}
