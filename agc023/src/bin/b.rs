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
        s: [Chars; n],
    }
    let mut result = 0usize;
    let mut b = vec![vec![' '; n]; n];
    for a in 0..n {
        for i in 0..n {
            for j in 0..n {
                b[(i + a) % n][j] = s[i][j];
            }
        }
        if (0..n).all(|i| (0..i).all(|j| b[i][j] == b[j][i])) {
            result += n;
        }
    }
    println!("{}", result);
}
