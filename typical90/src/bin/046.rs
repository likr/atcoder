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
        a: [[usize; n]; 3],
    }
    let m = 46;
    let mut count = vec![vec![0; m]; 3];
    for j in 0..3 {
        for i in 0..n {
            count[j][a[j][i] % m] += 1;
        }
    }
    let mut result = 0usize;
    for i in 0..m {
        for j in 0..m {
            for k in 0..m {
                if (i + j + k) % m == 0 {
                    result += count[0][i] * count[1][j] * count[2][k];
                }
            }
        }
    }
    println!("{}", result);
}
