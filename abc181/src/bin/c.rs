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
        xy: [(i64, i64); n],
    }
    for i in 0..n {
        let (xi, yi) = xy[i];
        for j in 0..i {
            let (xj, yj) = xy[j];
            for k in 0..j {
                let (xk, yk) = xy[k];
                if (xj - xi) * (yk - yi) == (xk - xi) * (yj - yi) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
