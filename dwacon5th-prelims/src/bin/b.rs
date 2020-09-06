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
        k: usize,
        a: [usize; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let mut b = vec![];
    for i in 1..=n {
        for j in 0..i {
            b.push(acc[i] - acc[j]);
        }
    }
    let mut result = 0;
    for i in (0..=40).rev() {
        let x = result | 1 << i;
        if b.iter().filter(|&bi| bi & x == x).count() >= k {
            result = x;
        }
    }
    println!("{}", result);
}
