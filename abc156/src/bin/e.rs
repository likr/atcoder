use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn inv(a: usize) -> usize {
    let m = M as i64;
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
        n: usize,
        k: usize,
    }

    let mut f = vec![1; 2 * n];
    let mut g = vec![1; 2 * n];
    for i in 1..f.len() {
        f[i] = i * f[i - 1] % M;
        g[i] = inv(f[i]);
    }
    let mut s = 0;
    for m in 0..=min(k, n - 1) {
        let c = f[n - 1] * g[n - 1 - m] % M * g[m] % M;
        let d = f[n] * g[n - m] % M * g[m] % M;
        s = (s + c * d % M) % M;
    }
    println!("{}", s);
}
