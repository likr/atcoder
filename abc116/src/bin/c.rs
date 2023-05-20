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
        mut n: usize,
        mut h: [usize; n],
    }
    h.dedup();
    n = h.len();
    h.push(0);
    h.reverse();
    h.push(0);
    h.reverse();
    let mut result = 0;
    for i in 1..=n {
        if h[i - 1] < h[i] && h[i] > h[i + 1] {
            result += h[i];
        }
    }
    for i in 1..=n {
        if h[i - 1] > h[i] && h[i] < h[i + 1] {
            result -= h[i];
        }
    }
    println!("{}", result);
}
