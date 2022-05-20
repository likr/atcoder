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
        mut k: usize,
    }
    let n = s.len();
    let mut result = 0;
    let mut j = 0;
    for i in 0..n {
        while j < n && (k > 0 || s[j] == 'X') {
            if s[j] == '.' {
                k -= 1;
            }
            j += 1;
        }
        result = max(result, j - i);
        if s[i] == '.' {
            k += 1;
        }
    }
    println!("{}", result);
}
