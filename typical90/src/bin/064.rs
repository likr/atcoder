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
        q: usize,
        a: [i64; n],
        lrv: [(Usize1, Usize1, i64); q],
    }
    let mut d = vec![0; n];
    let mut s = 0;
    for i in 1..n {
        d[i] = a[i] - a[i - 1];
        s += d[i].abs();
    }
    for i in 0..q {
        let (li, ri, vi) = lrv[i];
        if li > 0 {
            s -= d[li].abs();
            d[li] += vi;
            s += d[li].abs();
        }
        if ri + 1 < n {
            s -= d[ri + 1].abs();
            d[ri + 1] -= vi;
            s += d[ri + 1].abs();
        }
        println!("{}", s);
    }
}
