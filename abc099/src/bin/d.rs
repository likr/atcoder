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
        n: usize,
        m: usize,
        d: [[usize; m]; m],
        c: [[Usize1; n]; n],
    }
    let mut e = vec![vec![]; 3];
    for i in 0..n {
        for j in 0..n {
            e[(i + j) % 3].push(c[i][j]);
        }
    }
    let mut f = vec![vec![0; m]; 3];
    for k in 0..3 {
        for y in 0..m {
            for &x in &e[k] {
                f[k][y] += d[x][y];
            }
        }
    }
    let mut result = INF;
    for i in 0..m {
        for j in 0..m {
            if i == j {
                continue;
            }
            for k in 0..m {
                if i == k || j == k {
                    continue;
                }
                result = min(result, f[0][i] + f[1][j] + f[2][k]);
            }
        }
    }
    println!("{}", result);
}
