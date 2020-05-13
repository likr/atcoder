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
        a: [isize; n],
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let mut count = HashMap::new();
    for i in 0..=n {
        *count.entry(acc[i]).or_insert(0) += 1;
    }
    let mut s = 0usize;
    for &c in count.values() {
        s += c * (c - 1) / 2;
    }
    println!("{}", s);
}
