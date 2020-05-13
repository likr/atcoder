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
        a: [usize; n],
    }
    // for &ai in &a {
    //     eprintln!("{:024b}", ai);
    // }
    let m = 20;
    let mut bits = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            bits[i][j] = a[i] >> j & 1;
        }
    }

    let mut acc = vec![vec![0; m]; n + 1];
    for i in 0..n {
        for j in 0..m {
            acc[i + 1][j] += acc[i][j] + bits[i][j];
        }
    }

    let mut result = 0usize;
    let mut l = 1;
    let mut r = 1;
    while l <= n {
        while r <= n && (0..m).all(|j| acc[r][j] - acc[l - 1][j] <= 1) {
            r += 1;
        }
        result += r - l;
        l += 1;
    }

    println!("{}", result);
}
