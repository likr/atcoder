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
        m: usize,
        k: usize,
        ab: [(Usize1, Usize1); m],
        ph: [(Usize1, i64); k],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in ab.iter() {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    let mut heap = BinaryHeap::new();
    let mut h = vec![-1; n];
    for &(pi, hi) in ph.iter() {
        heap.push((hi, pi));
        h[pi] = hi;
    }
    while let Some((c, u)) = heap.pop() {
        if h[u] > c {
            continue;
        }
        for &v in graph[u].iter() {
            if h[u] - 1 > h[v] {
                h[v] = h[u] - 1;
                heap.push((h[v], v))
            }
        }
    }
    let mut ans = 0;
    let mut s = vec![];
    for u in 0..n {
        if h[u] >= 0 {
            ans += 1;
            s.push(format!("{}", u + 1));
        }
    }
    println!("{}", ans);
    println!("{}", s.join(" "));
}
