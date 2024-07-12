use ac_library::*;
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

fn rec(graph: &Vec<Vec<usize>>, u: usize, dp: &mut Vec<bool>, visited: &mut Vec<bool>) -> bool {
    if !visited[u] {
        dp[u] = graph[u].iter().any(|&v| rec(graph, v, dp, visited));
        visited[u] = true;
    }
    dp[u]
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut graph = SccGraph::new(n);
    for &(ui, vi) in uv.iter() {
        graph.add_edge(ui, vi);
    }
    let scc = graph.scc();
    let mut dsu = Dsu::new(n);
    for v in scc.iter() {
        for i in 1..v.len() {
            dsu.merge(v[i - 1], v[i]);
        }
    }
    let mut graph = vec![vec![]; n];
    for &(ui, vi) in uv.iter() {
        if dsu.leader(ui) != dsu.leader(vi) {
            graph[dsu.leader(ui)].push(dsu.leader(vi));
        }
    }
    let mut dp = vec![false; n];
    let mut visited = vec![false; n];
    for u in 0..n {
        if dsu.size(u) > 1 {
            dp[u] = true;
            visited[u] = true;
        }
    }
    let mut ans = 0usize;
    for u in 0..n {
        if rec(&graph, u, &mut dp, &mut visited) {
            ans += 1;
        }
    }
    debug!(dp);
    println!("{}", ans);
}
