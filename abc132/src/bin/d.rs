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

pub struct Combination {
    m: usize,
    f: Vec<usize>,
    g: Vec<usize>,
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
    pub fn permutations(&mut self, n: usize, k: usize) -> usize {
        for i in self.f.len()..=n {
            self.f.push(self.f[i - 1] * i % self.m);
            self.g.push(inv(self.f[i], self.m));
        }
        self.f[n] * self.g[n - k] % self.m
    }
    pub fn factorial(&mut self, n: usize) -> usize {
        for i in self.f.len()..=n {
            self.f.push(self.f[i - 1] * i % self.m);
            self.g.push(inv(self.f[i], self.m));
        }
        self.f[n]
    }
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut c = Combination::new(M);
    for i in 1..=k {
        if n + 1 < k || n + 1 < k + i {
            println!("0");
        } else {
            let x = c.combinations(dbg!(n + 1 - k), dbg!(n + 1 - k - i));
            let y = c.combinations(dbg!(k - 1), dbg!(k - i));
            eprintln!("{} {}", x, y);
            println!("{}", x * y % M);
        }
    }
}
