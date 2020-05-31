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
        a: [usize; n + 1],
    }
    let mut acc = a.clone();
    for i in (0..n).rev() {
        acc[i] += acc[i + 1];
    }

    let mut b = vec![0usize; n + 1];
    if a[0] > 1 {
        println!("-1");
        return;
    }
    b[0] = 1 - a[0];

    for i in 1..n {
        if 2 * b[i - 1] < a[i] {
            println!("-1");
            return;
        }
        b[i] = min(acc[i + 1], 2 * b[i - 1] - a[i]);
    }
    if n > 0 && 2 * b[n - 1] < a[n] {
        println!("-1");
        return;
    }
    // eprintln!("{:?}", acc);
    // eprintln!("{:?}", b);
    println!("{}", (0..=n).map(|i| a[i] + b[i]).sum::<usize>());
}
