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
    }
    let mut l = 0;
    let mut h = 1000000;
    while h - l > 1 {
        let m = (h + l) / 2;
        let mut b = 1;
        while b <= m {
            b *= 10;
        }
        debug!(l, m, h, m * b + m);
        if m * b + m <= n {
            l = m;
        } else {
            h = m;
        }
    }
    println!("{}", l);
}
