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
        csf: [(usize, usize, usize); n - 1],
    }
    for i in 0..n - 1 {
        let mut t = 0;
        for j in i..n - 1 {
            let (cj, sj, fj) = csf[j];
            let tj = if t < sj {
                sj
            } else if (t - sj) % fj == 0 {
                t
            } else {
                sj + fj * ((t - sj + fj - 1) / fj)
            };
            t = tj + cj;
        }
        println!("{}", t);
    }
    println!("0");
}
