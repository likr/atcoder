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

fn f(n: usize) -> usize {
    let mut d = vec![];
    let mut m = n;
    while m > 0 {
        d.push(m % 10);
        m /= 10;
    }

    let mut s = vec![];
    s.push((n, d.iter().sum::<usize>()));

    for i in 1..d.len() {
        d[i - 1] = 9;
        s.push((
            d.iter()
                .enumerate()
                .map(|(i, di)| 10usize.pow(i as u32) * di)
                .sum::<usize>(),
            d.iter().sum::<usize>(),
        ));
    }
    for i in 0..d.len() {
        if (0..d.len()).all(|j| s[i].0 * s[j].1 <= s[j].0 * s[i].1) {
            return s[i].0;
        }
    }
    return INF;
}

fn main() {
    input! {
        k: usize,
    }
    let mut n = 1;
    for _ in 0..k {
        println!("{}", n);
        n = f(n + 1);
    }
}
