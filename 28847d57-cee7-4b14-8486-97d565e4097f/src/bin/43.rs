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
        lrv: [(Usize1, Usize1, i64); q]
    }
    let mut d = vec![0; n - 1];
    let mut s = 0;
    for i in 1..n {
        d[i - 1] = a[i - 1] - a[i];
        s += d[i - 1].abs();
    }
    for &(li, ri, vi) in lrv.iter() {
        if li > 0 {
            s -= d[li - 1].abs();
            d[li - 1] -= vi;
            s += d[li - 1].abs();
        }
        if ri < n - 1 {
            s -= d[ri].abs();
            d[ri] += vi;
            s += d[ri].abs();
        }
        println!("{}", s);
    }
}
