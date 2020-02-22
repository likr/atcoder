use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
        h: usize,
        w: usize,
        c: [[usize; 10]; 10],
        a: [[isize; w]; h],
    }
    let mut d = vec![vec![INF; 10]; 10];
    for i in 0..10 {
        for j in 0..10 {
            d[i][j] = c[i][j];
        }
    }
    for k in 0..10 {
        for i in 0..10 {
            for j in 0..10 {
                d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
            }
        }
    }
    // eprintln!("{:?}", d);
    let mut s = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == -1 {
                continue;
            }
            s += d[a[i][j] as usize][1];
        }
    }
    println!("{}", s);
}
