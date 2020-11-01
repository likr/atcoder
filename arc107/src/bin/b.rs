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
        n: i64,
        k: i64,
    }
    let mut result = 0;
    for x in 2..=2 * n {
        let y = x - k;
        if 2 <= y && y <= 2 * n {
            let sx = if x <= n + 1 { x - 1 } else { 2 * n + 1 - x };
            let sy = if y <= n + 1 { y - 1 } else { 2 * n + 1 - y };
            result += sx * sy;
        }
    }
    println!("{}", result);
}
