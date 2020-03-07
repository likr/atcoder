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

fn solve(xv: &Vec<(isize, isize)>, c: isize) -> isize {
    let n = xv.len();

    let mut score = vec![0isize; n];
    score[0] = xv[0].1 - xv[0].0;
    for i in 1..n {
        let (xj, _) = xv[i - 1];
        let (xi, vi) = xv[i];
        score[i] = score[i - 1] + vi - (xi - xj);
    }
    // eprintln!("{:?}", score);

    let mut score2 = vec![0isize; n];
    score2[n - 1] = xv[n - 1].1 - 2 * (c - xv[n - 1].0);
    for i in (0..n - 1).rev() {
        let (xj, _) = xv[i + 1];
        let (xi, vi) = xv[i];
        score2[i] = score2[i + 1] + vi - 2 * (xj - xi);
    }
    // eprintln!("{:?}", score2);

    let mut rev = vec![0isize; n];
    for i in (0..n - 1).rev() {
        rev[i] = max(rev[i + 1], score2[i + 1]);
    }

    let mut result = 0;
    for i in 0..n {
        result = max(result, score[i] + max(0, rev[i]));
    }
    result
}

fn main() {
    input! {
        n: usize,
        c: isize,
        mut xv: [(isize, isize); n],
    }
    let a = solve(&xv, c);
    for i in 0..n {
        xv[i].0 = c - xv[i].0
    }
    xv.reverse();
    let b = solve(&xv, c);
    println!("{}", max(a, b));
}
