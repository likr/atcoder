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
        mut w: usize,
        mut ab: [(usize, usize); n],
    }
    ab.sort_by_key(|&(ai, _)| ai);
    ab.reverse();
    let mut s = 0;
    for i in 0..n {
        let (ai, bi) = ab[i];
        if w < bi {
            s += ai * w;
            w = 0;
        } else {
            s += ai * bi;
            w -= bi;
        }
    }
    println!("{}", s);
}
