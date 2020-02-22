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
        d: usize,
        a: [[Usize1; w]; h],
        q: usize,
        lr: [(Usize1, Usize1); q],
    }
    let n = h * w;
    let mut indices = vec![(0, 0); n];
    for i in 0..h {
        for j in 0..w {
            indices[a[i][j]] = (i, j);
        }
    }
    let mut cost = vec![0; n];
    for k in 0..n - d {
        let (i, j) = indices[k];
        let (x, y) = indices[k + d];
        cost[k + d] = max(x, i) - min(x, i) + max(y, j) - min(y, j);
    }
    let mut acc = vec![0; n];
    for i in 0..d {
        for j in (i + d..n).step_by(d) {
            acc[j] = cost[j] + acc[j - d];
        }
    }
    for &(li, ri) in &lr {
        println!("{}", acc[ri] - acc[li]);
    }
}
