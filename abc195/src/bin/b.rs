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
        a: usize,
        b: usize,
        mut w: usize,
    }
    w *= 1000;
    let mut x = INF;
    let mut y = 0;
    for i in 1..=1000000 {
        let c = w / i;
        if w % i == 0 {
            if a <= c && c <= b {
                x = min(x, i);
                y = max(y, i);
            }
        } else {
            if a <= c && c < b {
                x = min(x, i);
                y = max(y, i);
            }
        }
    }
    if x == INF || y == 0 {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", x, y);
    }
}
