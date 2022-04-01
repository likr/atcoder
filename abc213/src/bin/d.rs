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

fn dfs(graph: &Vec<Vec<usize>>, u: usize, visited: &mut HashSet<usize>, result: &mut Vec<usize>) {
    result.push(u + 1);
    for &v in graph[u].iter() {
        if !visited.contains(&v) {
            visited.insert(v);
            dfs(graph, v, visited, result);
            result.push(u + 1);
        }
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1]
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in ab.iter() {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    for u in 0..n {
        graph[u].sort();
    }
    let mut visited = HashSet::new();
    visited.insert(0);
    let mut result = vec![];
    dfs(&graph, 0, &mut visited, &mut result);
    println!(
        "{}",
        result
            .into_iter()
            .map(|u| format!("{}", u))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
