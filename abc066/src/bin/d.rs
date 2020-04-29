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
        a: [usize; n + 1],
    }
    let n = n + 1;
    let mut used = HashSet::new();
    let mut b = 0;
    for i in 0..n {
        if used.contains(&a[i]) {
            b = a[i];
            break;
        }
        used.insert(a[i]);
    }

    let left = a.iter().position(|&ai| ai == b).unwrap();
    let right = n - a.iter().rposition(|&ai| ai == b).unwrap() - 1;
    let m = left + right;

    let mut f = vec![1; n + 1];
    let mut g = vec![1; n + 1];
    for i in 2..=n {
        f[i] = (f[i - 1] * i) % M;
        g[i] = inv(f[i]);
    }

    println!("{}", n - 1);
    for l in 2..=n {
        let x = f[n] * g[l] % M * g[n - l] % M;
        let y = if l <= m + 1 {
            f[m] * g[l - 1] % M * g[m + 1 - l] % M
        } else {
            0
        };
        println!("{}", (x + M - y) % M);
    }
}
