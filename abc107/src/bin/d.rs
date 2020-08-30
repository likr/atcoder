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
        a: [usize; n],
    }

    let msize = n * (n + 1) / 2;
    let mut l = 0;
    let mut r = INF;
    while r - l > 1 {
        let m = (r + l) / 2;
        let b = a
            .iter()
            .map(|&ai| if ai >= m { 1 } else { -1 })
            .collect::<Vec<isize>>();
        let mut acc_b = vec![0; n + 1];
        for i in 0..n {
            acc_b[i + 1] = acc_b[i] + b[i];
        }
        let acc_b_min = *acc_b.iter().min().unwrap();
        for i in 0..=n {
            acc_b[i] -= acc_b_min;
        }
        let acc_b_max = *acc_b.iter().max().unwrap();

        let mut bit = BIT::new(acc_b_max as usize + 1);
        let mut count = 0;
        for i in 0..=n {
            count += bit.query(acc_b[i] as usize) as usize;
            bit.update(acc_b[i] as usize, 1);
        }
        // dbg!((l, r, m, count));
        if count >= (msize + 1) / 2 {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}", a.iter().filter(|&&ai| ai <= l).max().unwrap());
}
