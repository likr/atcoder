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

fn is_palindrome(x: usize) -> bool {
    let mut digits = vec![];
    let mut x = x;
    while x > 0 {
        digits.push(x % 10);
        x /= 10;
    }
    (0..digits.len()).all(|i| digits[i] == digits[digits.len() - 1 - i])
}

fn main() {
    input! {
        n: usize,
    }
    let mut ans = 1;
    for i in 1.. {
        if i * i * i > n {
            break;
        }
        if is_palindrome(i * i * i) {
            ans = i * i * i;
        }
    }
    println!("{}", ans);
}
