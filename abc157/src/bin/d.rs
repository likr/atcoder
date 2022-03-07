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
        k: usize,
        ab: [(Usize1, Usize1); m],
        cd: [(Usize1, Usize1); k],
    }
    let mut uf = UnionFind::new(n);
    for &(ai, bi) in ab.iter() {
        uf.union(ai, bi);
    }
    let mut count = vec![0; n];
    for i in 0..n {
        count[uf.find(i)] += 1;
    }
    let mut result = vec![0; n];
    for i in 0..n {
        result[i] = count[uf.find(i)] - 1;
    }
    for &(ai, bi) in ab.iter() {
        result[ai] -= 1;
        result[bi] -= 1;
    }
    for &(ci, di) in cd.iter() {
        if uf.find(ci) == uf.find(di) {
            result[ci] -= 1;
            result[di] -= 1;
        }
    }
    println!(
        "{}",
        result
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
