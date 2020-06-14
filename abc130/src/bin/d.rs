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
    for i in 1..=n {
        acc[i] = acc[i - 1] + a[i - 1];
    }
    // eprintln!("{:?}", acc);
    let mut l = 0;
    let mut r = 0;
    let mut result = 0usize;
    while l < n {
        while r < n && acc[r] - acc[l] < k {
            r += 1;
        }
        // eprintln!("{} {}", l, r);
        if acc[r] - acc[l] >= k {
            result += n - r + 1;
        }
        l += 1;
    }
    println!("{}", result);
}
