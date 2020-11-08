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
        s: Chars,
    }
    let mut t = s.clone();
    t.sort();
    loop {
        if s != t && (0..n).any(|i| s[i] != t[n - i - 1]) {
            println!("{}", t.iter().collect::<String>());
            return;
        }
        if !t.next_permutation() {
            break;
        }
    }
    println!("None");
}
