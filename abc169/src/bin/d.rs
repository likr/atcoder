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
        mut n: usize,
    }
    let mut primes = vec![true; 1000001];
    primes[0] = false;
    primes[1] = false;
    for i in 2..primes.len() {
        if primes[i] {
            for j in 2.. {
                if i * j >= primes.len() {
                    break;
                }
                primes[i * j] = false;
            }
        }
    }
    let mut result = 0usize;
    for i in 2..primes.len() {
        let mut count = 0;
        while n % i == 0 {
            count += 1;
            n /= i;
        }
        let mut s = 0;
        loop {
            if (s + 1) * (s + 2) / 2 > count {
                break;
            }
            s += 1;
        }
        result += s;
    }
    if n > 1 {
        result += 1;
    }
    println!("{}", result);
}
