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
        xy: [(f64, f64); n],
    }
    for i in 0..n {
        let (xi, yi) = xy[i];
        let mut ans = 0;
        let mut max_d = 0.;
        for j in 0..n {
            let (xj, yj) = xy[j];
            let d = (xi - xj).hypot(yi - yj);
            if d > max_d {
                ans = j;
                max_d = d;
            }
        }
        println!("{}", ans + 1);
    }
}
