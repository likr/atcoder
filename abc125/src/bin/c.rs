use num::Integer;
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
    let mut left = a.clone();
    let mut right = a.clone();
    for i in 1..n {
        left[i] = left[i - 1].gcd(&left[i]);
    }
    for i in (1..n).rev() {
        right[i - 1] = right[i].gcd(&right[i - 1]);
    }
    let mut ans = max(right[1], left[n - 2]);
    for i in 1..n - 1 {
        ans = max(ans, left[i - 1].gcd(&right[i + 1]));
    }
    println!("{}", ans);
}
