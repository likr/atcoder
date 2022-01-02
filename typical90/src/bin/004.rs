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
        a: [[u64; w]; h],
    }
    let mut rows = vec![0; h];
    for i in 0..h {
        let mut s = 0;
        for j in 0..w {
            s += a[i][j];
        }
        rows[i] = s;
    }
    let mut cols = vec![0; w];
    for j in 0..w {
        let mut s = 0;
        for i in 0..h {
            s += a[i][j];
        }
        cols[j] = s;
    }
    for i in 0..h {
        println!(
            "{}",
            (0..w)
                .map(|j| format!("{}", rows[i] + cols[j] - a[i][j]))
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}
