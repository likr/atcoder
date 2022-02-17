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
        k: i64,
        a: [i64; n],
    }
    let mut ng = -1;
    let mut ok = a.iter().max().unwrap() + 1;
    while ok - ng > 1 {
        let x = (ok + ng) / 2;
        let mut count = 0;
        for i in 0..n {
            if a[i] >= x {
                count += a[i] - x + 1;
            }
        }
        if count > k {
            ng = x;
        } else {
            ok = x;
        }
    }
    debug!((ng, ok));
    let mut result = 0;
    if ng < 0 {
        for i in 0..n {
            result += a[i] * (a[i] + 1) / 2;
        }
    } else {
        let mut count = 0;
        for i in 0..n {
            if a[i] >= ok {
                count += a[i] - ng;
                result += (a[i] - ng) * (a[i] + ng + 1) / 2;
            }
        }
        result += ng * (k - count);
    }
    println!("{}", result);
}
