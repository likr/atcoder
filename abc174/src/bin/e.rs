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
        k: usize,
        a: [usize; n],
    }
    let sum_a = a.iter().sum::<usize>();
    if k >= sum_a {
        println!("1");
        return;
    }
    let mut ng = 1;
    let mut ok = INF;
    while ok - ng > 1 {
        let m = (ok + ng) / 2;
        let mut count = 0;
        for i in 0..n {
            if a[i] > m {
                count += a[i] / m;
            }
        }
        if count > k {
            ng = m;
        } else {
            ok = m;
        }
    }
    println!("{}", ok);
}
