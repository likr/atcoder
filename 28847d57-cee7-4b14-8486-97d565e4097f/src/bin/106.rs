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

fn nth_ancestor(ancestors: &Vec<Vec<Option<usize>>>, u: usize, n: usize) -> Option<usize> {
    let m = ancestors.len();
    let mut u = u;
    for i in 0..m {
        if n & 1 << i > 0 {
            if let Some(v) = ancestors[i][u] {
                u = v;
            } else {
                return None;
            }
        }
    }
    Some(u)
}

fn main() {
    input! {
        n: usize,
        xy: [(Usize1, Usize1); n - 1],
        q: usize,
        ab: [(Usize1, Usize1); q],
    }
    let mut graph = vec![vec![]; n];
    for &(u, v) in xy.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut parent = vec![None; n];
    let mut h = vec![INF; n];
    let mut queue = VecDeque::new();
    h[0] = 0;
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if h[v] == INF {
                parent[v] = Some(u);
                h[v] = h[u] + 1;
                queue.push_back(v);
            }
        }
    }
    let m = 20;
    let mut ancestors = vec![vec![None; n]; m];
    for i in 0..n {
        ancestors[0][i] = parent[i];
    }
    for k in 1..m {
        for i in 0..n {
            ancestors[k][i] = if let Some(j) = ancestors[k - 1][i] {
                ancestors[k - 1][j]
            } else {
                None
            };
        }
    }
    for &(u, v) in ab.iter() {
        let mut u0 = u;
        let mut v0 = v;
        if h[u0] > h[v0] {
            u0 = nth_ancestor(&ancestors, u0, h[u0] - h[v0]).unwrap();
        } else {
            v0 = nth_ancestor(&ancestors, v0, h[v0] - h[u0]).unwrap();
        }
        if u0 == v0 {
            println!("{}", max(h[u], h[v]) - min(h[u], h[v]) + 1);
            continue;
        }
        let mut ng = 0;
        let mut ok = 1 << m;
        while ok - ng > 1 {
            let d = (ok + ng) / 2;
            let u1 = nth_ancestor(&ancestors, u0, d);
            let v1 = nth_ancestor(&ancestors, v0, d);
            if u1 == v1 {
                ok = d;
            } else {
                ng = d;
            }
        }
        let w = nth_ancestor(&ancestors, u0, ok).unwrap();
        println!("{}", h[u] - h[w] + h[v] - h[w] + 1);
    }
}
