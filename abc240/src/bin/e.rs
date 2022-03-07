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

fn dfs(graph: &Vec<Vec<usize>>, u: usize, l: &mut Vec<usize>, r: &mut Vec<usize>) {
    let mut l_last = l[u] - 1;
    for &v in graph[u].iter() {
        if l[v] == INF {
            l[v] = l_last + 1;
            dfs(graph, v, l, r);
            l_last = r[v];
        }
    }
    r[u] = l_last + 1;
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
    let mut l = vec![INF; n];
    l[0] = 1;
    let mut r = vec![INF; n];
    dfs(&graph, 0, &mut l, &mut r);
    for i in 0..n {
        println!("{} {}", l[i], r[i]);
    }
}
