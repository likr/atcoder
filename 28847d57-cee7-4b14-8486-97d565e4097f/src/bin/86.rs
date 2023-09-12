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
        n: i64,
        m: usize,
        k: i64,
        mut ab: [(i64, i64); m],
    }
    ab.push((n + 1, 0));
    let mut ans = 0;
    let mut s = 0;
    for i in 0..m {
        s += ab[i].1;
        let d = ab[i + 1].0 - ab[i].0;
        ans += min(d, max(0, s - k));
        s = max(0, s - d);
    }
    println!("{}", ans);
}
