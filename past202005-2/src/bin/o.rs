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
const INF: isize = std::isize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
        b: [isize; n],
        r: [isize; 3],
    }
    let mut s = vec![vec![vec![1; 3]; m + 1]; n];
    let mut t = vec![vec![0; 3 * m + 1]; n];
    for i in 0..n {
        for k in 0..3 {
            s[i][0][k] = a[i];
            for j in 1..=m {
                s[i][j][k] = (b[i] * s[i][j - 1][k].pow(k as u32 + 1)) % r[k];
            }
            s[i][0][k] = 0;
        }
        t[i][0] = a[i];
        for j in 1..=3 * m {
            t[i][j] = if INF / t[i][j - 1] < b[i] {
                INF
            } else {
                b[i] * t[i][j - 1]
            };
        }
        t[i][0] = 0;
    }
    for k in 0..3 {
        for i in 0..n {
            for j in 0..=m {
                print!("{:8}", s[i][j][k]);
            }
            println!("");
        }
    }
    for i in 0..n {
        for j in 0..=3 * m {
            print!("{:8}", t[i][j]);
        }
        println!("");
    }
}
