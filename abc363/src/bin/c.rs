use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
        mut s: Chars,
    }
    s.sort();
    let mut set = HashSet::new();
    loop {
        if (0..=n - k).all(|i| (0..k).any(|j| s[i + j] != s[i + k - 1 - j])) {
            set.insert(s.clone());
        }
        if !s.next_permutation() {
            break;
        }
    }
    println!("{}", set.len());
}
