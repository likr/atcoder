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
const M: usize = 998244353;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn inv(a: usize, m: usize) -> usize {
    let m = m as i64;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1;
    let mut v = 0;
    let mut tmp;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        tmp = a;
        a = b;
        b = tmp;
        u -= t * v;
        tmp = u;
        u = v;
        v = tmp;
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u as usize;
}

fn main() {
    input! {
        mut s: Chars,
    }
    s.reverse();
    let s = s
        .into_iter()
        .map(|d| (d as u8 - '0' as u8) as usize)
        .collect::<Vec<_>>();
    let n = s.len();
    let mut b = 1;
    for _ in 1..n {
        b = b * 2 % M;
    }
    let c = b;
    let mut result = 0;
    for i in 0..n {
        result = (result + s[i] * b % M) % M;
        debug!(result, b);
        b = ((b * 5 % M) + (c * inv(2, M))) % M;
    }
    println!("{}", result);
}
