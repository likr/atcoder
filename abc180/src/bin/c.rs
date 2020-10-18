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

pub fn factors(n: usize) -> Vec<usize> {
    let mut result = vec![];
    for d in 1.. {
        if d * d > n {
            break;
        }
        if n % d == 0 {
            result.push(d);
            result.push(n / d);
        }
    }
    result.sort();
    result.dedup();
    result
}

fn main() {
    input! {
        n: usize,
    }
    for f in factors(n) {
        println!("{}", f);
    }
}
