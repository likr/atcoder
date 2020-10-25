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
    }
    let mut a = vec![];
    let mut a0 = 3;
    while a0 < n {
        a.push(a0);
        a0 *= 3;
    }
    let mut b = vec![];
    let mut b0 = 5;
    while b0 < n {
        b.push(b0);
        b0 *= 5;
    }
    debug!(a, b);
    for i in 0..a.len() {
        let ai = a[i];
        for j in 0..b.len() {
            let bi = b[j];
            if ai + bi == n {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
    println!("-1");
}
