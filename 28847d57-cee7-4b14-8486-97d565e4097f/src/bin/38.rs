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

fn dfs(graph: &Vec<Vec<usize>>, u: usize, visited: &mut Vec<bool>, ans: &mut Vec<usize>) {
    ans.push(u);
    for &v in graph[u].iter() {
        if !visited[v] {
            visited[v] = true;
            dfs(graph, v, visited, ans);
            ans.push(u);
        }
    }
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
    for i in 0..n {
        graph[i].sort();
    }
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut ans = vec![];
    dfs(&graph, 0, &mut visited, &mut ans);
    println!(
        "{}",
        ans.iter()
            .map(|u| format!("{}", u + 1))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
