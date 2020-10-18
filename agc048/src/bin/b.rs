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
        a: [i64; n],
        b: [i64; n],
    }
    let mut odd = (0..n).step_by(2).collect::<Vec<_>>();
    let mut even = (1..n).step_by(2).collect::<Vec<_>>();
    odd.sort_by_key(|&i| Reverse(b[i] - a[i]));
    even.sort_by_key(|&i| Reverse(b[i] - a[i]));
    let mut result = a.iter().sum::<i64>();
    let mut s = result;
    for i in 0..n / 2 {
        s = s + b[odd[i]] - a[odd[i]] + b[even[i]] - a[even[i]];
        result = max(result, s);
    }
    println!("{}", result);
}
