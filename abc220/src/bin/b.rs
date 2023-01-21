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
        k: usize,
        mut a: usize,
        mut b: usize,
    }
    let mut a10 = 0;
    let mut b10 = 0;
    let mut base = 1;
    while a > 0 || b > 0 {
        a10 += (a % 10) * base;
        b10 += (b % 10) * base;
        a /= 10;
        b /= 10;
        base *= k;
    }
    println!("{}", a10 * b10);
}
