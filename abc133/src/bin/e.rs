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
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut degree = vec![0usize; n];
    for &(ai, bi) in &ab {
        degree[ai] += 1;
        degree[bi] += 1;
    }
    let mut x = vec![0; n];
    for &(ai, bi) in &ab {
        x[ai] += degree[bi];
        x[bi] += degree[ai];
    }
    eprintln!("{:?}", x);
    let mut result = k;
    for i in 0..n {
        result = result * (k + 1 - x[i]) % M;
    }
    println!("{}", result);
}
