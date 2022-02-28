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
    let mut m = 0;
    while 1 << m < k {
        m += 1;
    }

    let mut d = vec![vec![0; n]; m + 1];
    for j in 0..n {
        d[0][j] = a[j];
    }
    for i in 0..m {
        for j in 0..n {
            d[i + 1][j] = d[i][j] + d[i][(j + d[i][j]) % n];
        }
    }
    let mut x = 0;
    let mut result = 0;
    for i in 0..=m {
        if 1 << i & k > 0 {
            result += d[i][x];
            x = result % n;
        }
    }
    println!("{}", result);
}
