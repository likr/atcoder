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
        m: Usize1,
        mut a: [usize; n],
        c: [usize; n],
    }
    a.reverse();
    let mut x = vec![vec![0; n]; n];
    for i in 0..n {
        x[0][i] = c[i];
    }
    for i in 1..n {
        x[i][i - 1] = (1 << 32) - 1;
    }
    let mut x = vec![x];
    for b in 1.. {
        if 1 << b > m {
            break;
        }
        let mut y = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    y[i][j] ^= x[b - 1][i][k] & x[b - 1][k][j];
                }
            }
        }
        x.push(y);
    }

    for b in 0..x.len() {
        // eprintln!("{:?}", a);
        // eprintln!("{:?}", x[b]);
        if 1 << b & m > 0 {
            a = (0..n)
                .map(|i| {
                    let mut s = 0;
                    for j in 0..n {
                        s ^= x[b][i][j] & a[j];
                    }
                    s
                })
                .collect::<Vec<_>>();
        }
    }
    println!("{}", a[n - 1]);
}
