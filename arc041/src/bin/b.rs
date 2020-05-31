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
        m: usize,
        b: [Chars; n],
    }
    let mut b = b
        .into_iter()
        .map(|row| {
            (0..m)
                .map(|j| row[j] as usize - '0' as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut a = vec![vec![0; m]; n];
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            a[i][j] = *[b[i - 1][j], b[i + 1][j], b[i][j - 1], b[i][j + 1]]
                .iter()
                .min()
                .unwrap();
            b[i - 1][j] -= a[i][j];
            b[i + 1][j] -= a[i][j];
            b[i][j - 1] -= a[i][j];
            b[i][j + 1] -= a[i][j];
        }
    }
    for i in 0..n {
        println!(
            "{}",
            (0..m).map(|j| format!("{}", a[i][j])).collect::<String>()
        );
    }
}
