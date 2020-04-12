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
        s: Chars,
    }
    let mut left = vec![0; n + 1];
    for i in 1..=n {
        left[i] = left[i - 1];
        if s[i - 1] == '#' {
            left[i] += 1;
        }
    }
    let mut right = vec![0; n + 1];
    for i in (0..n).rev() {
        right[i] = right[i + 1];
        if s[i] == '.' {
            right[i] += 1;
        }
    }
    // eprintln!("{:?}", left);
    // eprintln!("{:?}", right);
    let mut result = INF;
    for i in 0..=n {
        result = min(result, left[i] + right[i]);
    }
    println!("{}", result);
}
