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
        a: [usize; n],
        b: [usize; m],
    }
    let mut ng = 0;
    let mut ok = INF;
    while ok - ng > 1 {
        let x = (ok + ng) / 2;
        let mut c1 = 0;
        for i in 0..n {
            if a[i] <= x {
                c1 += 1;
            }
        }
        let mut c2 = 0;
        for j in 0..m {
            if b[j] >= x {
                c2 += 1;
            }
        }
        if c1 >= c2 {
            ok = x;
        } else {
            ng = x;
        }
    }
    println!("{}", ok);
}
