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
    }
    let mut count = 0;
    for i in k..=n + 1 {
        let min_s = i * (i - 1) / 2;
        let max_s = if i >= n {
            n * (n + 1) / 2
        } else {
            n * (n + 1) / 2 - (n - i) * (n - i + 1) / 2
        };
        // eprintln!("{} {}", max_s, min_s);
        count = (count + (max_s - min_s + 1)) % M;
    }
    println!("{}", count);
}
