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
        k: usize,
    }
    let n = s.len();
    let mut dot_count = vec![0; n + 1];
    for i in 0..n {
        if s[i] == '.' {
            dot_count[i + 1] = 1;
        }
    }
    for i in 0..n {
        dot_count[i + 1] += dot_count[i];
    }
    let mut ans = 0;
    let mut j = 0;
    for i in 0..n {
        while j < n && (s[j] == 'x' || dot_count[j + 1] - dot_count[i] <= k) {
            j += 1;
        }
        ans = max(ans, j - i);
    }
    println!("{}", ans);
}
