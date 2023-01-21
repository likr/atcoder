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
    }
    if k < min(n, 5) {
        println!("0");
        return;
    }
    let mut result = 1usize;
    for i in 0..min(2, n) {
        result = (result * (k - i)) % M;
    }
    if n >= 2 {
        let mut b = k - 2;
        let mut m = n - 2;
        while m > 0 {
            if m % 2 == 1 {
                result = (result * b) % M;
            }
            b = (b * b) % M;
            m /= 2;
        }
    }
    println!("{}", result);
}
