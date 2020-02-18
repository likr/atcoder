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
        q: usize,
        lr: [(Usize1, Usize1); m],
        pq: [(Usize1, Usize1); q],
    }
    let mut d = vec![vec![0; n]; n];
    for &(li, ri) in &lr {
        d[li][ri] += 1;
    }
    // eprintln!("{:?}", d);
    let mut acc = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            acc[i][j] = d[i - 1][j - 1] + acc[i - 1][j] + acc[i][j - 1] - acc[i - 1][j - 1];
        }
    }
    // eprintln!("{:?}", acc);
    for &(pi, qi) in &pq {
        println!(
            "{}",
            acc[qi + 1][qi + 1] + acc[pi][pi] - acc[pi][qi + 1] - acc[qi + 1][pi]
        );
    }
}
