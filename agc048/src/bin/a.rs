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
        q: usize,
    }
    let t = "atcoder".chars().collect::<Vec<_>>();
    let m = t.len();
    for _ in 0..q {
        input! {
            s: Chars,
        }
        let n = s.len();
        let mut result = INF;
        for i in 0..m {
            for j in i..n {
                if t[i] < s[j] {
                    result = min(result, j - i);
                }
            }
            if i < n && s[i] != t[i] {
                break;
            }
        }
        if n > m && (0..m).all(|i| s[i] == t[i]) {
            println!("0");
        } else if result == INF {
            println!("-1");
        } else {
            println!("{}", result);
        }
    }
}
