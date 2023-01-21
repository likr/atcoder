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
    let mut m = 1;
    while m * m <= n {
        m += 1;
    }
    let mut prime = vec![true; m + 1];
    for i in 2..=m {
        if prime[i] {
            for j in 2.. {
                if i * j > m {
                    break;
                }
                prime[i * j] = false;
            }
        }
    }
    let mut count = 0;
    for d in 2..=m {
        if prime[d] {
            while n % d == 0 {
                n /= d;
                count += 1;
            }
        }
    }
    if n > 1 {
        count += 1;
    }
    let mut result = 0;
    while 1 << result < count {
        result += 1;
    }
    println!("{}", result);
}
