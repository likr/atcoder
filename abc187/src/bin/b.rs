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
    let mut ans = 0usize;
    for j in 1..n {
        let (xj, yj) = xy[j];
        for i in 0..j {
            let (xi, yi) = xy[i];
            if -(xi - xj).abs() <= (yi - yj).abs() && (yi - yj).abs() <= (xi - xj).abs() {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}