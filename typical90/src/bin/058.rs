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
    let m = 100000;
    let mut doubling = vec![vec![INF; m]; 60];
    for i in 0..m {
        let mut x = i;
        let mut s = 0;
        while x > 0 {
            s += x % 10;
            x /= 10;
        }
        doubling[0][i] = (i + s) % m;
    }
    for j in 1..60 {
        for i in 0..m {
            doubling[j][i] = doubling[j - 1][doubling[j - 1][i]];
        }
    }

    let mut current = n;
    for j in 0..60 {
        if 1 << j & k > 0 {
            current = doubling[j][current];
        }
    }
    println!("{}", current);
}
