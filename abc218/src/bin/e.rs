use petgraph::unionfind::UnionFind;
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
        abc: [(Usize1, Usize1, i64); m],
    }
    let mut edges = BinaryHeap::new();
    for i in 0..m {
        let (ai, bi, ci) = abc[i];
        edges.push((Reverse(ci), ai, bi));
    }
    let mut uf = UnionFind::new(n);
    let mut result = 0;
    while let Some((Reverse(ci), ai, bi)) = edges.pop() {
        if !uf.union(ai, bi) && ci > 0 {
            result += ci;
        }
    }
    println!("{}", result);
}
