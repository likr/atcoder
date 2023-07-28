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
        c: [usize; 9],
    }
    let c_min = c.iter().min().unwrap();
    let digits = n / c_min;
    let mut x = n % c_min;
    let mut result = vec![];
    for _ in 0..digits {
        let k = (0..9).filter(|&i| c[i] <= c_min + x).max().unwrap();
        result.push(format!("{}", k + 1));
        x -= c[k] - c_min;
    }
    println!("{}", result.join(""));
}
