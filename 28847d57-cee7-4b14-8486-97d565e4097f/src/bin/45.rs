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
        b: [[i64; w]; h],
    }
    let mut c = 0;
    for i in 1..h {
        for j in 1..w {
            let d = a[i - 1][j - 1] - b[i - 1][j - 1];
            a[i - 1][j - 1] -= d;
            a[i - 1][j] -= d;
            a[i][j - 1] -= d;
            a[i][j] -= d;
            c += d.abs();
        }
    }
    if a == b {
        println!("Yes");
        println!("{}", c);
    } else {
        println!("No");
    }
}
