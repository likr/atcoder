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

fn sum(ab: &Vec<(usize, u64)>, mask: usize) -> u64 {
    ab.iter()
        .filter(|&(ai, _)| ai | mask == mask)
        .map(|&(_, bi)| bi)
        .sum()
}

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, u64); n],
    }
    let ab = ab
        .into_iter()
        .filter(|&(ai, _)| ai <= k)
        .collect::<Vec<_>>();
    let mut m = 0;
    let mut k2 = k;
    while k2 > 0 {
        m += 1;
        k2 /= 2;
    }
    let mut result = sum(&ab, k);
    for i in 0..m {
        if k & (1 << i) > 0 {
            let x = k & !(1 << i);
            let x = x | ((1 << i) - 1);
            let s = sum(&ab, x);
            result = max(s, result);
            // eprintln!("{:b} {}", x, s);
        }
    }
    println!("{}", result);
}
