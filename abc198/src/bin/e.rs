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
    colors: &Vec<usize>,
    u: usize,
    count: &mut Vec<usize>,
    result: &mut Vec<Option<bool>>,
) {
    count[colors[u]] += 1;
    for &v in graph[u].iter() {
        if result[v].is_none() {
            result[v] = Some(count[colors[v]] == 0);
            dfs(graph, colors, v, count, result);
        }
    }
    count[colors[u]] -= 1;
}

fn main() {
    input! {
        n: usize,
        c: [usize; n],
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..n - 1 {
        let (ai, bi) = ab[i];
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    let mut count = vec![0; 100001];
    let mut result = vec![None; n];
    result[0] = Some(true);
    dfs(&graph, &c, 0, &mut count, &mut result);
    for i in 0..n {
        if let Some(f) = result[i] {
            if f {
                println!("{}", i + 1);
            }
        }
    }
}
