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
        a: [usize; n],
    }
    let mut count = vec![0; 200001];
    for i in 0..n {
        count[a[i]] += 1;
    }
    let mut result = n * (n - 1) * (n - 2) / 6;
    for i in 0..count.len() {
        if count[i] >= 2 {
            result -= count[i] * (count[i] - 1) / 2 * (n - count[i]);
        }
        if count[i] >= 3 {
            result -= count[i] * (count[i] - 1) * (count[i] - 2) / 6;
        }
    }
    println!("{}", result);
}
