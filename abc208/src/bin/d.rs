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
        abc: [(Usize1, Usize1, usize); m],
    }
    let mut distance = vec![vec![INF; n]; n];
    for i in 0..n {
        distance[i][i] = 0;
    }
    for i in 0..m {
        let (ai, bi, ci) = abc[i];
        distance[ai][bi] = ci;
    }
    let mut result = 0;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                distance[i][j] = min(distance[i][j], distance[i][k] + distance[k][j]);
                if distance[i][j] != INF {
                    result += distance[i][j]
                }
            }
        }
    }
    println!("{}", result);
}
