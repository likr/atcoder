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
        a: [[usize; n]; n],
    }
    let mut b = vec![vec![true; n]; n];
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if a[i][j] <= a[i][k] + a[k][j] {
                    b[i][j] = false;
                }
            }
        }
    }
    let mut s = 0;
    for i in 0..n {
        for j in 0..n {
            if b[i][j] {
                s += a[i][j];
            }
        }
    }
    println!("{}", s);
}
