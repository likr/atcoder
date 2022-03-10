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
const INF: usize = 100000000;
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
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let mut ok = INF;
    let mut ng = 0;
    while ok - ng > 1 {
        let k = (ok + ng) / 2;
        if a + k * b <= d * k * c {
            ok = k;
        } else {
            ng = k;
        }
    }
    if ok == INF {
        println!("-1");
    } else {
        println!("{}", ok);
    }
}
