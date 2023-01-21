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
        h: usize,
        w: usize,
        mut a: [[i64; w]; h],
        mut b: [[i64; w]; h],
    }
    let mut s = 0;
    for i in 1..h {
        for j in 1..w {
            if a[i - 1][j - 1] < b[i - 1][j - 1] {
                let d = b[i - 1][j - 1] - a[i - 1][j - 1];
                a[i - 1][j - 1] += d;
                a[i - 1][j] += d;
                a[i][j - 1] += d;
                a[i][j] += d;
                s += d;
            } else if b[i - 1][j - 1] < a[i - 1][j - 1] {
                let d = a[i - 1][j - 1] - b[i - 1][j - 1];
                b[i - 1][j - 1] += d;
                b[i - 1][j] += d;
                b[i][j - 1] += d;
                b[i][j] += d;
                s += d;
            }
        }
    }
    if (0..h).all(|i| (0..w).all(|j| a[i][j] == b[i][j])) {
        println!("Yes");
        println!("{}", s);
    } else {
        println!("No");
    }
}
