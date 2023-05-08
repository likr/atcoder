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
    for i in 0..s.len() {
        if i % 2 == 0 {
            if 'A' <= s[i] && s[i] <= 'Z' {
                println!("No");
                return;
            }
        } else {
            if 'a' <= s[i] && s[i] <= 'z' {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
