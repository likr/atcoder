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
        m: usize,
        k: usize,
    }

    let mut f = vec![1; n * m + 1];
    let mut g = vec![1; n * m + 1];
    for i in 1..n * m + 1 {
        f[i] = i * f[i - 1] % M;
        g[i] = inv(f[i]);
    }

    let mut s = 0;
    for i in 1..=(n - 1) + (m - 1) {
        let fi = (f[n * m - 2] * g[k - 2] % M) * g[n * m - k] % M;
        let fi = i * fi % M;
        if n > i {
            let c = (n - i) * m % M;
            let si = c * fi % M;
            s = (s + si) % M;
        }
        if m > i {
            let c = n * (m - i) % M;
            let si = c * fi % M;
            s = (s + si) % M;
        }
        for j in if i > n { i - n } else { 0 } + 1..min(i, m) {
            if n + j > i && m > j {
                let c = 2 * (n + j - i) * (m - j) % M;
                let si = c * fi % M;
                s = (s + si) % M;
            }
        }
    }
    println!("{}", s);
}
