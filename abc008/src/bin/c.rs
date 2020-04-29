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
        c: [usize; n],
    }
    let mut factors = vec![0; n];
    for i in 0..n {
        for j in 0..n {
            if c[i] % c[j] == 0 {
                factors[i] += 1;
            }
        }
    }
    let mut s = 0f64;
    for i in 0..n {
        let b = factors[i];
        let a = (b + 1) / 2;
        s += a as f64 / b as f64;
    }
    println!("{}", s);
}
