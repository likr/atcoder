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
        cp: [(Usize1, u64); n],
        q: usize,
        lr: [(Usize1, Usize1); q],
    }
    let mut acc = vec![vec![0; n + 1]; 2];
    for i in 1..=n {
        let (ci, pi) = cp[i - 1];
        acc[0][i] = acc[0][i - 1];
        acc[1][i] = acc[1][i - 1];
        acc[ci][i] += pi;
    }
    for i in 0..q {
        let (li, ri) = lr[i];
        println!(
            "{} {}",
            acc[0][ri + 1] - acc[0][li],
            acc[1][ri + 1] - acc[1][li]
        );
    }
}
