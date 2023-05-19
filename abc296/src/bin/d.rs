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
    }
    if n * n < m {
        println!("-1");
        return;
    }
    let mut k = 1;
    while k * k < m {
        k += 1;
    }
    let mut result = INF;
    for i in 1..=k {
        let mut ng = 1;
        let mut ok = m;
        while ok - ng > 1 {
            let mid = (ok + ng) / 2;
            if mid * i < m {
                ng = mid;
            } else {
                ok = mid;
            }
        }
        if i * ok >= m && ok <= n {
            result = result.min(i * ok);
        }
    }
    println!("{}", result);
}
