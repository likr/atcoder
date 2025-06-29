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
        m: usize,
        l: [usize; n],
    }
    let mut ok = INF;
    let mut ng = l.iter().max().unwrap() - 1;
    while ok - ng > 1 {
        let w = (ok + ng) / 2;
        let mut s = l[0];
        let mut rows = 1;
        for i in 1..n {
            if s + 1 + l[i] <= w {
                s = s + 1 + l[i];
            } else {
                s = l[i];
                rows += 1;
            }
        }
        if rows <= m {
            ok = w;
        } else {
            ng = w;
        }
    }
    println!("{}", ok);
}
