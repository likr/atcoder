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
        n: usize,
    }
    let mut factors = vec![];
    let mut x = n;
    for d in 2.. {
        if d * d > n {
            break;
        }
        while x % d == 0 {
            factors.push(d);
            x /= d;
        }
    }
    if x > 1 {
        factors.push(x);
    }
    let mut ans = 0;
    let mut m = 1;
    while factors.len() > m {
        ans += 1;
        m *= 2;
    }
    println!("{}", ans);
}
