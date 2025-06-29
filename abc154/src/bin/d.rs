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
        k: usize,
        p: [usize; n],
    }
    let mut exp = vec![0.; n];
    for i in 0..n {
        exp[i] = (p[i] + 1) as f64 / 2.0;
    }
    let mut s = 0.0;
    for i in 1..k {
        s += exp[i - 1];
    }
    let mut ans = 0.0;
    for i in (k - 1)..n {
        s += exp[i];
        if s > ans {
            ans = s;
        }
        s -= exp[i + 1 - k]
    }
    println!("{}", ans);
}
