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

fn f(t: usize, a: usize, b: usize) -> f64 {
    b as f64 * t as f64 + a as f64 / (t as f64 + 1.0).sqrt()
}

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let mut left = 0;
    let mut right = a + 1;
    for _ in 0..100 {
        let c1 = (left * 2 + right) / 3;
        let c2 = (left + right * 2) / 3;
        if f(c1, a, b) > f(c2, a, b) {
            left = c1;
        } else {
            right = c2
        }
    }
    let mut result = f((left + right) / 2, a, b);
    if f(left, a, b) < result {
        result = f(left, a, b);
    }
    if f(right, a, b) < result {
        result = f(right, a, b);
    }
    println!("{}", result);
}
