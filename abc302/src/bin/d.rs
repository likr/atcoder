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
        d: i64,
        a: [i64; n],
        mut b: [i64; m],
    }
    b.sort();
    let mut result = 0;
    for i in 0..n {
        let k = b.upper_bound(&(a[i] + d));
        if k > 0 && a[i] - b[k - 1] <= d {
            debug!(a[i], b[k - 1]);
            result = max(result, a[i] + b[k - 1]);
        }
    }
    if result == 0 {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
