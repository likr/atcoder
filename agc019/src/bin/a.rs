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
        p: [usize; 4],
        mut n: usize,
    }
    let mut ignore = vec![false; 4];
    for i in 1..4 {
        for j in 0..i {
            ignore[i] = ignore[i] || p[j] * 2usize.pow((i - j) as u32) <= p[i];
        }
    }
    eprintln!("{:?}", ignore);
    let mut result = 0;
    if !ignore[3] {
        result += p[3] * (n / 2);
        n %= 2;
    }
    for i in (0..3).rev() {
        if !ignore[i] {
            result += p[i] * n * 2usize.pow((2 - i) as u32);
            break;
        }
    }
    println!("{}", result);
}
