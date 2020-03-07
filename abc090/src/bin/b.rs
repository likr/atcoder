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
        a: usize,
        b: usize,
    }
    let mut count = 0;
    for i in a..=b {
        let s = format!("{}", i).chars().collect::<Vec<_>>();
        let n = s.len();
        if (0..n / 2).all(|j| s[j] == s[n - 1 - j]) {
            count += 1;
        }
    }
    println!("{}", count);
}
