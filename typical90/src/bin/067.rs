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

fn to_digits(mut x: usize, digits: &mut Vec<usize>, base: usize) {
    digits.clear();
    while x > 0 {
        digits.push(x % base);
        x /= base;
    }
}

fn from_digits(digits: &Vec<usize>, base: usize) -> usize {
    let mut x = 0;
    let mut b = 1;
    for &d in digits.iter() {
        x += d * b;
        b *= base;
    }
    x
}

fn main() {
    input! {
        n: Chars,
        k: usize,
    }
    let mut digits = n
        .iter()
        .rev()
        .map(|&d| d as u8 as usize - '0' as u8 as usize)
        .collect::<Vec<_>>();
    let mut result = from_digits(&digits, 8);
    for _ in 0..k {
        to_digits(result, &mut digits, 9);
        for i in 0..digits.len() {
            if digits[i] == 8 {
                digits[i] = 5;
            }
        }
        result = from_digits(&digits, 8)
    }
    to_digits(result, &mut digits, 8);
    println!("{}", from_digits(&digits, 10));
}
