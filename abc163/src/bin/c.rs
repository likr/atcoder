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
        a: [Usize1; n - 1],
    }
    let mut count = vec![0; n];
    for i in 1..n {
        count[a[i - 1]] += 1;
    }
    for i in 0..n {
        println!("{}", count[i]);
    }
}
