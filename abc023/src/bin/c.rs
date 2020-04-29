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
        r: usize,
        c: usize,
        k: usize,
        n: usize,
        rc: [(Usize1, Usize1); n],
    }
    let mut count_row = vec![0usize; r];
    let mut count_col = vec![0usize; c];
    for &(ri, ci) in &rc {
        count_row[ri] += 1;
        count_col[ci] += 1;
    }
    let mut h = vec![0usize; k + 1];
    for i in 0..r {
        if count_row[i] <= k {
            h[count_row[i]] += 1;
        }
    }
    let mut w = vec![0usize; k + 1];
    for i in 0..c {
        if count_col[i] <= k {
            w[count_col[i]] += 1;
        }
    }

    let mut result = 0usize;
    for x in 0..=k {
        let y = k - x;
        result += h[x] * w[y];
    }
    for &(ri, ci) in &rc {
        let s = count_row[ri] + count_col[ci];
        if s == k {
            result -= 1;
        } else if s == k + 1 {
            result += 1;
        }
    }
    println!("{}", result);
}
