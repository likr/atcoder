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
        k: Usize1,
    }
    let m = n + k;
    let mut f = vec![1; m + 1];
    let mut g = vec![1; m + 1];
    for i in 1..=m {
        f[i] = (f[i - 1] * i) % M;
        g[i] = inv(f[i]);
    }

    let mut s = 0usize;
    for i in 0..n {
        s = (s + f[k + i] * g[k] % M * g[i] % M) % M;
    }
    println!("{}", s);
}
