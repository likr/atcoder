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
        m: usize,
        b: [[Usize1; m]; n],
    }
    for i in 0..n {
        for j in 1..m {
            if b[i][j - 1] + 1 != b[i][j] || b[i][j - 1] % 7 > b[i][j] % 7 {
                println!("No");
                return;
            }
        }
    }
    for i in 1..n {
        for j in 0..m {
            if b[i - 1][j] + 7 != b[i][j] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
