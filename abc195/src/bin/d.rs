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
        q: usize,
        mut wv: [(usize, usize); n],
        mut x: [usize; m],
        lr: [(Usize1, Usize1); q],
    }
    wv.sort_by_key(|&(_, vi)| vi);
    wv.reverse();
    for k in 0..q {
        let (lk, rk) = lr[k];
        let mut used = vec![false; m];
        for j in lk..=rk {
            used[j] = true;
        }
        let mut s = 0;
        for i in 0..n {
            let (wi, vi) = wv[i];
            let mut b = INF;
            let mut index = INF;
            for j in 0..m {
                if !used[j] && wi <= x[j] && x[j] < b {
                    b = x[j];
                    index = j;
                }
            }
            if b != INF {
                s += vi;
                used[index] = true;
            }
        }
        println!("{}", s);
    }
}
