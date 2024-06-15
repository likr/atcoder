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
    let mut distance = vec![INF; n];
    distance[0] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if distance[v] == INF {
                distance[v] = distance[u] + 1;
                queue.push_back(v);
            }
        }
    }
    let mut c = vec![0; n];
    for &(ti, ei, xi) in tex.iter() {
        let (u, v) = ab[ei];
        if ti == 1 {
            if distance[u] < distance[v] {
                c[0] += xi;
                c[v] -= xi;
            } else {
                c[u] += xi;
            }
        } else {
            if distance[u] < distance[v] {
                c[v] += xi;
            } else {
                c[0] += xi;
                c[u] -= xi;
            }
        }
    }
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if !visited[v] {
                c[v] += c[u];
                visited[v] = true;
                queue.push_back(v);
            }
        }
    }
    for i in 0..n {
        println!("{}", c[i]);
    }
}
