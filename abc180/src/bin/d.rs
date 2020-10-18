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
        mut x: usize,
        y: usize,
        a: usize,
        b: usize,
    }
    let mut result = 0;
    while x * (a - 1) <= b && a * x < y {
        x *= a;
        result += 1;
    }
    debug!(x, result);
    result += (y - 1 - x) / b;
    println!("{}", result);
}
