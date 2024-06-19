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
        s: Chars,
    }
    let n = s.len();
    let mut ans = n * (n - 1) / 2;
    let mut count = HashMap::new();
    for &c in s.iter() {
        *count.entry(c).or_insert(0) += 1;
    }
    if count.values().any(|&c| c >= 2) {
        ans += 1;
    }
    for &c in count.values() {
        ans -= c * (c - 1) / 2;
    }
    println!("{}", ans);
}
