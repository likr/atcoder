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
        m: usize,
        p: usize,
        a: [usize; n],
        mut b: [usize; m],
    }
    b.sort();
    let mut acc_b = vec![0; m + 1];
    for j in 0..m {
        acc_b[j + 1] = b[j] + acc_b[j];
    }
    let mut s = 0;
    for i in 0..n {
        let k = b.lower_bound_by_key(&p, |bj| a[i] + bj);
        s += p * (m - k) + a[i] * k + acc_b[k];
    }
    println!("{}", s);
}
