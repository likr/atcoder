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
    }
    let mut ok = 2000000001;
    let mut ng = 0;
    while ok - ng > 1 {
        let x = (ok + ng) / 2;
        if x * x + x - 2 * n - 2 > 0 {
            ok = x;
        } else {
            ng = x;
        }
    }
    debug!(ok, ng);
    println!("{}", n + 2 - ok);
}
