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
        a: [[Usize1]; k],
    }
    let mut count = vec![0; n];
    for i in 0..k {
        for &aij in &a[i] {
            count[aij] += 1;
        }
    }
    let mut result = 0;
    for i in 0..n {
        if count[i] == 0 {
            result += 1;
        }
    }
    println!("{}", result);
}
