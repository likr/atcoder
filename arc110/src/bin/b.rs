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
        t: Chars,
    }

    let result = if (0..3).any(|i| (0..n).all(|j| t[j] == (if j % 3 == i { '0' } else { '1' }))) {
        if n == 1 && t[0] == '1' {
            20000000000
        } else if n == 2 && t[0] == '1' && t[1] == '1' {
            10000000000
        } else if t[n - 1] == '0' {
            10000000001 - t.iter().filter(|&&ti| ti == '0').count()
        } else {
            10000000000 - t.iter().filter(|&&ti| ti == '0').count()
        }
    } else {
        0
    };
    println!("{}", result);
}
