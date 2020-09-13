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

pub fn modpow(x: usize, y: usize, m: usize) -> usize {
    let mut result = 1;
    let mut a = x;
    let mut b = y;
    while b > 0 {
        if b % 2 == 1 {
            result = result * a % m;
        }
        a = a * a % m;
        b /= 2;
    }
    result
}

fn main() {
    input! {
        n: usize,
    }
    if n < 2 {
        println!("0");
        return;
    }
    let x = modpow(10, n, M);
    let y = modpow(9, n, M);
    let z = modpow(8, n, M);
    println!("{}", ((x + M - (2 * y) % M) % M + z) % M);
}
