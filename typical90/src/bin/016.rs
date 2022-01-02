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
        a: i64,
        b: i64,
        c: i64,
    }
    let mut result = INF as i64;
    for x in 0..=9999 {
        if a * x > n {
            break;
        }
        for y in 0..=9999 {
            if a * x + b * y > n {
                break;
            }
            let r = n - a * x - b * y;
            if r % c == 0 {
                let z = r / c;
                result = min(result, x + y + z);
            }
        }
    }
    println!("{}", result);
}
