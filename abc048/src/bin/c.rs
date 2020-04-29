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
        x: isize,
        a: [isize; n],
    }
    let mut b = vec![0; n];
    for i in 1..n {
        b[i] = max(0, a[i - 1] + a[i] - x);
    }
    let mut c = vec![0; n];
    for i in 1..n {
        c[i] = max(0, b[i] - min(a[i - 1], c[i - 1]));
    }
    // eprintln!("{:?}", b);
    // eprintln!("{:?}", c);
    println!("{}", c.iter().sum::<isize>());
}
