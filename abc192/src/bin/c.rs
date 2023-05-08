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
        k: usize,
    }
    let mut count = vec![0; 10];
    for _ in 0..k {
        for i in 0..10 {
            count[i] = 0;
        }
        while n > 0 {
            count[n % 10] += 1;
            n /= 10;
        }
        let mut g1 = 0;
        for i in (0..10).rev() {
            for _ in 0..count[i] {
                g1 = g1 * 10 + i;
            }
        }
        let mut g2 = 0;
        for i in 1..10 {
            for _ in 0..count[i] {
                g2 = g2 * 10 + i;
            }
        }
        n = g1 - g2;
    }
    println!("{}", n);
}
