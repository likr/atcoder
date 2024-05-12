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
        xy: [(i64, i64); n],
    }
    let mut p = vec![vec![]; 4];
    for i in 0..n {
        let (xi, yi) = xy[i];
        p[(xi + yi) as usize % 2].push(xi - yi);
        p[2 + (xi + yi) as usize % 2].push(xi + yi);
    }
    let mut ans = 0;
    for i in 0..4 {
        p[i].sort();
        for j in 0..p[i].len() {
            ans += (2 * j as i64 - p[i].len() as i64 + 1) * p[i][j];
        }
    }
    println!("{}", ans / 2);
}
