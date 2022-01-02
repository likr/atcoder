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

fn check(digits: &Vec<usize>) -> bool {
    let mut stack = 0;
    for i in 0..digits.len() {
        if digits[i] == 0 {
            stack += 1;
        } else {
            if stack == 0 {
                return false;
            }
            stack -= 1;
        }
    }
    stack == 0
}

fn main() {
    input! {
        n: usize,
    }
    if n % 2 == 1 {
        return;
    }
    let mut digits = vec![0; n];
    for x in 0..=2usize.pow(n as u32) {
        for i in 0..n {
            digits[i] = (x >> i) % 2;
        }
        digits.reverse();
        if check(&digits) {
            for i in 0..n {
                if digits[i] == 0 {
                    print!("(");
                } else {
                    print!(")");
                }
            }
            println!("");
        }
    }
}
