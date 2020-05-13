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

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    }
    let mut odd = 0;
    let mut even = 0;
    for &ai in &a {
        if ai % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }

    let mut binom = vec![vec![1, 1]];
    for _ in 2..=odd {
        let b = &binom[binom.len() - 1];
        let mut c = vec![];
        c.push(1);
        for i in 1..b.len() {
            c.push(b[i - 1] + b[i]);
        }
        c.push(1);
        binom.push(c);
    }

    let b = &binom[binom.len() - 1];
    let s = 2usize.pow(even as u32);
    let mut result = 0;
    for i in 0..=odd {
        if i % 2 == p {
            result += s * b[i];
        }
    }
    println!("{}", result);
}
