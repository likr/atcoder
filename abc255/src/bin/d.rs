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
        q: usize,
        mut a: [usize; n],
        x: [usize; q],
    }
    a.sort();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] += acc[i] + a[i];
    }
    for i in 0..q {
        let xi = x[i];
        let k = a.lower_bound(&xi);
        let mut result = 0;
        if k > 0 {
            result += xi * k - acc[k];
        }
        if k < n {
            result += acc[n] - acc[k] - xi * (n - k);
        }
        println!("{}", result);
    }
}
