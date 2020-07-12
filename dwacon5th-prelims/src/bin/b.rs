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
    b.sort();
    let mut count = vec![0; 60];
    let mut result = 0;
    for &bj in &b {
        for i in 0..count.len() {
            if 1 << i & bj >= 1 {
                count[i] += 1;
            }
        }
        if (0..count.len()).all(|i| 1 << i & bj == 0 || count[i] >= k) {
            result = bj;
        }
    }
    println!("{}", result);
}
