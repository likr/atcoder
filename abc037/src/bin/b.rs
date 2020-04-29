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
        q: usize,
        lrt: [(Usize1, Usize1, usize); q],
    }
    let mut a = vec![0; n];
    for &(li, ri, ti) in &lrt {
        for j in li..=ri {
            a[j] = ti;
        }
    }
    for j in 0..n {
        println!("{}", a[j]);
    }
}
