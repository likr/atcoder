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
        r1: i64,
        c1: i64,
        r2: i64,
        c2: i64,
    }
    if (r1, c1) == (r2, c2) {
        println!("0");
    } else if r1 + c1 == r2 + c2 || r1 - c1 == r2 - c2 || (r1 - r2).abs() + (c1 - c2).abs() <= 3 {
        println!("1");
    } else if (r1 + c1) % 2 == (r2 + c2) % 2 {
        println!("2");
    } else if (-3i64..=3).any(|x| {
        (-3i64..=3).any(|y| {
            x.abs() + y.abs() <= 3
                && (r1 + x + c1 + y == r2 + c2
                    || r1 + x - c1 - y == r2 - c2
                    || (r1 + x - r2).abs() + (c1 + y - c2).abs() <= 3)
        })
    }) {
        println!("2");
    } else {
        println!("3");
    }
}
