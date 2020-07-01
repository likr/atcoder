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

pub struct Combination {
    m: usize,
    f: Vec<usize>,
    g: Vec<usize>,
}

impl Combination {
    pub fn new(m: usize) -> Combination {
        Combination {
            m,
            f: vec![1],
            g: vec![1],
        }
    }
    pub fn combinations(&mut self, n: usize, k: usize) -> usize {
        for i in self.f.len()..=n {
            self.f.push(self.f[i - 1] * i % self.m);
            self.g.push(inv(self.f[i], self.m));
        }
        self.f[n] * self.g[k] % self.m * self.g[n - k] % self.m
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut c = Combination::new(M);
    if n <= k {
        let a = k % n;
        let b = n - a;
        println!("{}", c.combinations(a + b, a));
    } else {
        println!("{}", c.combinations(n + k - 1, n - 1));
    }
}
