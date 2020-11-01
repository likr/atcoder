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
        q: usize,
        ab: [(Usize1, Usize1); n - 1],
        px: [(Usize1, usize); q],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in &ab {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    let mut c = vec![0; n];
    for &(pi, xi) in &px {
        c[pi] += xi;
    }
    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut visited = vec![false; n];
    visited[0] = true;
    while let Some(u) = queue.pop_front() {
        for &v in &graph[u] {
            if !visited[v] {
                visited[v] = true;
                c[v] += c[u];
                queue.push_back(v);
            }
        }
    }
    println!(
        "{}",
        c.iter()
            .map(|ci| format!("{}", ci))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
