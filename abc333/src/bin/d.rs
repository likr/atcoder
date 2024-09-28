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

fn dfs(u: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> usize {
    visited[u] = true;
    let mut c = 1;
    for &v in graph[u].iter() {
        if !visited[v] {
            c += dfs(v, graph, visited);
        }
    }
    c
}

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut ans = 0;
    for &u in graph[0].iter() {
        let c = dfs(u, &graph, &mut visited);
        debug!(u, c);
        ans = max(ans, c);
    }
    println!("{}", n - ans);
}
