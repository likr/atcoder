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
        a: Chars,
    }
    let a = a
        .into_iter()
        .map(|c| c as isize - '0' as isize)
        .collect::<Vec<isize>>();
    let mut b = vec![vec![0; n]; n];
    for j in 0..n {
        b[0][j] = a[j];
    }
    for i in 1..n {
        for j in 0..n - i {
            b[i][j] = (b[i - 1][j] - b[i - 1][j + 1]).abs();
        }
    }
    for i in 0..n {
        eprintln!("{:?}", b[i]);
    }
}
