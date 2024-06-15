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
        mut ab: [(usize, usize); n],
    }
    let mut sa = 0;
    for i in 0..n {
        sa += ab[i].0;
    }
    ab.sort_by_key(|&(ai, bi)| ai * 2 + bi);
    ab.reverse();
    let mut sb = 0;
    for i in 0..n {
        let (ai, bi) = ab[i];
        sa -= ai;
        sb += ai + bi;
        if sb > sa {
            println!("{}", i + 1);
            return;
        }
    }
}
