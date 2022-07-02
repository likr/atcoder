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
        s: Chars,
    }
    let n = s.len();

    let mut result1 = 0;
    for i in 0..n {
        if (i % 2 == 0 && s[i] == '0') || (i % 2 == 1 && s[i] == '1') {
            result1 += 1;
        }
    }
    let mut result2 = 0;
    for i in 0..n {
        if (i % 2 == 0 && s[i] == '1') || (i % 2 == 1 && s[i] == '0') {
            result2 += 1;
        }
    }
    println!("{}", min(result1, result2));
}
