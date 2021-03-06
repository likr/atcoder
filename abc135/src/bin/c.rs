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
        mut a: [usize; n + 1],
        mut b: [usize; n],
    }
    let mut count = 0usize;
    for i in 0..n {
        if b[i] > a[i] {
            count += a[i];
            b[i] -= a[i];
            a[i] = 0;
        } else {
            count += b[i];
            a[i] -= b[i];
            b[i] = 0;
        }
        if b[i] > a[i + 1] {
            count += a[i + 1];
            b[i] -= a[i + 1];
            a[i + 1] = 0;
        } else {
            count += b[i];
            a[i + 1] -= b[i];
            b[i] = 0;
        }
    }
    debug!(a);
    debug!(b);
    println!("{}", count);
}
