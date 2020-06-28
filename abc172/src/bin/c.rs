use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut acc_a = vec![0; n + 1];
    for i in 0..n {
        acc_a[i + 1] = a[i] + acc_a[i];
    }
    let mut acc_b = vec![0; m + 1];
    for i in 0..m {
        acc_b[i + 1] = b[i] + acc_b[i];
    }
    // eprintln!("{:?}", acc_a);
    // eprintln!("{:?}", acc_b);
    let mut result = 0;
    for i in 0..=n {
        let t = acc_a[i];
        if t <= k {
            let j = acc_b.upper_bound(&(k - t)) - 1;
            // eprintln!("{} {} {} {}", i, j, acc_a[i], acc_b[j]);
            result = max(result, i + j);
        }
    }
    println!("{}", result);
}
