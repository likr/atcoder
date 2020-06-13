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
    }
    let m = 2usize.pow(n as u32);
    input! {
        a: [usize; m],
    }

    let mut candidate = vec![vec![]; n];
    for j in 0..m {
        candidate[0].push((a[j], j));
    }
    for i in 1..n {
        for j in (0..candidate[i - 1].len()).step_by(2) {
            let (aj, j) = max(candidate[i - 1][j], candidate[i - 1][j + 1]);
            candidate[i].push((aj, j));
        }
    }
    let mut result = vec![0; m];
    for i in 0..n {
        for &(_, j) in &candidate[i] {
            result[j] = i + 1;
        }
    }
    for j in 0..m {
        println!("{}", result[j]);
    }
}
