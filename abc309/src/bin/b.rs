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
        mut a: [Chars; n]
    }
    let a00 = a[0][0];
    for i in 1..n {
        a[i - 1][0] = a[i][0];
    }
    for i in 1..n {
        a[n - 1][i - 1] = a[n - 1][i];
    }
    for i in 1..n {
        a[n - i][n - 1] = a[n - i - 1][n - 1];
    }
    for i in 1..n {
        a[0][n - i] = a[0][n - i - 1];
    }
    a[0][1] = a00;
    for i in 0..n {
        println!("{}", a[i].iter().collect::<String>());
    }
}
