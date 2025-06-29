use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::Ext;

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
    let mut ts = HashSet::new();
    loop {
        ts.insert(s.clone());
        if !s.next_permutation() {
            break;
        }
    }
    debug!(ts);
    let mut ans = 0;
    for t in ts.iter() {
        if !(0..=n - k).any(|i| (0..k).all(|j| t[i + j] == t[i + k - 1 - j])) {
            debug!(t);
            ans += 1;
        }
    }
    println!("{}", ans);
}
