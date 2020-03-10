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

fn factorial(n: usize) -> usize {
    let mut f = 1;
    for i in 1..=n {
        f = (f * i) % M;
    }
    f
}

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
        r: usize,
        c: usize,
        x: usize,
        y: usize,
        d: usize,
        l: usize,
    }
    let g_d = inv(factorial(d));
    let g_l = inv(factorial(l));
    let mut s = 0usize;
    for &ii in &[0, 1] {
        for &jj in &[0, 1] {
            for &kk in &[0, 1] {
                for &ll in &[0, 1] {
                    if x <= ii + jj || y <= kk + ll {
                        continue;
                    }
                    let w = x - ii - jj;
                    let h = y - kk - ll;
                    if h * w < d + l {
                        continue;
                    }
                    let f_hw = factorial(h * w);
                    let g_s = inv(factorial(h * w - d - l));
                    if (ii + jj + kk + ll) % 2 == 0 {
                        s = (s + ((f_hw * g_d % M) * g_l % M) * g_s % M) % M;
                    } else {
                        s = (s + M - ((f_hw * g_d % M) * g_l % M) * g_s % M) % M;
                    }
                }
            }
        }
    }
    let n = (r - x + 1) * (c - y + 1);
    println!("{}", s * n % M);
}
