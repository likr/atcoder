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
        a: usize,
        x: usize,
        m: usize,
    }
    if a == 1 {
        println!("{}", x % m);
        return;
    }
    if m == 1 {
        println!("0");
        return;
    }
    let mut ax = 1;
    let mut b = a % m;
    for i in 0..60 {
        if x & 1 << i > 0 {
            ax = (ax * b) % m;
        }
        b = b * b % m;
    }
    debug!(ax);
    println!("{}", ((ax + m - 1) % m) * inv(a - 1, m) % m);
}
