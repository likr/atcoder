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

pub trait BITOperator {
    fn unit() -> Self;
    fn apply(a: &Self, b: &Self) -> Self;
}

pub struct BIT<T: BITOperator> {
    buffer: Vec<T>,
}

impl<T: BITOperator> BIT<T> {
    pub fn new(n: usize) -> BIT<T> {
        let mut buffer = vec![];
        for _ in 0..n {
            buffer.push(T::unit());
        }
        BIT { buffer }
    }

    pub fn update(&mut self, i: usize, v: T) {
        let mut x = i + 1;
        let mut base = 1;
        while x <= self.buffer.len() {
            self.buffer[x - 1] = T::apply(&v, &self.buffer[x - 1]);
            while x & base == 0 {
                base *= 2;
            }
            x += base;
        }
    }

    pub fn query(&self, i: usize) -> T {
        let mut v = T::unit();
        let mut x = i + 1;
        let mut base = 1;
        while x > 0 {
            v = T::apply(&v, &self.buffer[x - 1]);
            while x & base == 0 {
                base *= 2;
            }
            x -= base;
        }
        v
    }
}

impl BITOperator for isize {
    fn unit() -> isize {
        0
    }

    fn apply(a: &isize, b: &isize) -> isize {
        a + b
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [Usize1; n],
        mut lr: [(usize, usize); q],
    }

    let mut indices = (0..q).collect::<Vec<_>>();
    indices.sort_by_key(|&i| lr[i].1);

    let mut last_index = vec![INF; n];
    let mut prev = vec![INF; n];
    for i in 0..n {
        prev[i] = last_index[c[i]];
        last_index[c[i]] = i;
    }

    let mut result = vec![0; q];
    let mut bit = BIT::new(n + 1);
    let mut k = 1;
    for &i in &indices {
        let (li, ri) = lr[i];
        while k <= ri {
            bit.update(k, 1);
            if prev[k - 1] != INF {
                bit.update(prev[k - 1] + 1, -1);
            }
            k += 1;
        }
        result[i] = bit.query(ri) - bit.query(li - 1);
    }
    for i in 0..q {
        println!("{}", result[i]);
    }
}
