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
        s: Chars,
    }
    let mut intervals = vec![];
    let mut i = 0;
    while i < n {
        let mut j = i;
        while j < n && s[i] == s[j] {
            j += 1;
        }
        intervals.push(j - i);
        i = j;
    }
    let mut result = (n - 1) * n / 2;
    for v in intervals {
        result -= (v - 1) * v / 2;
    }
    println!("{}", result);
}
