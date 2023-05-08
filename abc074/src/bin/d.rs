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
        mut a: [[usize; n]; n],
    }
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if a[i][j] > a[i][k] + a[k][j] {
                    println!("-1");
                    return;
                }
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == a[i][k] + a[k][j] && a[i][k] > 0 && a[k][j] > 0 {
                    a[i][j] = INF;
                }
            }
        }
    }
    let mut result = 0;
    for i in 0..n {
        for j in 0..i {
            if a[i][j] != INF {
                result += a[i][j];
            }
        }
    }
    println!("{}", result);
}
