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
        q: usize,
        tex: [(usize, Usize1, i64); q],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in ab.iter() {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    let mut queue = VecDeque::new();
    let mut h = vec![INF; n];
    queue.push_back(0);
    h[0] = 0;
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if h[v] == INF {
                queue.push_back(v);
                h[v] = h[u] + 1;
            }
        }
    }
    let mut c = vec![0; n];
    for &(ti, ei, xi) in tex.iter() {
        let (ai, bi) = ab[ei];
        if ti == 1 {
            if h[ai] < h[bi] {
                c[0] += xi;
                c[bi] -= xi;
            } else {
                c[ai] += xi;
            }
        } else {
            if h[ai] < h[bi] {
                c[bi] += xi;
            } else {
                c[0] += xi;
                c[ai] -= xi;
            }
        }
    }
    let mut queue = VecDeque::new();
    let mut visited = vec![false; n];
    queue.push_back(0);
    visited[0] = true;
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if !visited[v] {
                queue.push_back(v);
                visited[v] = true;
                c[v] += c[u];
            }
        }
    }
    for u in 0..n {
        println!("{}", c[u]);
    }
}
