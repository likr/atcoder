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
        c: usize,
        a: [Usize1; n],
    }
    let mut indices = vec![vec![0]; c];
    for i in 0..n {
        indices[a[i]].push(i + 1);
    }
    for j in 0..c {
        indices[j].push(n + 1);
    }
    let total = n * (n + 1) / 2;
    for j in 0..c {
        let mut s = 0;
        for k in 1..indices[j].len() {
            let m = indices[j][k] - indices[j][k - 1];
            s += m * (m - 1) / 2;
        }
        println!("{}", total - s);
    }
}
