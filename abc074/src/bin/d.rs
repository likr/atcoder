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
    for i in 0..n {
        for j in 0..n {
            b[i][j] = (0..n).all(|k| i == k || j == k || a[i][j] < a[i][k] + a[k][j]);
        }
    }
    let mut s = 0;
    for i in 0..n {
        for j in i..n {
            if b[i][j] {
                s += a[i][j];
            }
        }
    }
    let mut d = vec![vec![INF; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    for i in 0..n {
        for j in 0..n {
            if b[i][j] {
                d[i][j] = a[i][j];
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
            }
        }
    }
    // eprintln!("{:?}", a);
    // eprintln!("{:?}", d);
    for i in 0..n {
        for j in 0..n {
            if d[i][j] != a[i][j] {
                println!("-1");
                return;
            }
        }
    }
    println!("{}", s);
}
