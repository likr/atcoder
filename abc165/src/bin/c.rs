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

fn dfs(
    x: &mut Vec<usize>,
    d: usize,
    n: usize,
    m: usize,
    scores: &Vec<Vec<Vec<(usize, usize)>>>,
) -> usize {
    if d == n {
        let mut result = 0;
        for i in 1..n {
            for j in 0..i {
                for &(ci, di) in &scores[j][i] {
                    if x[i] - x[j] == ci {
                        result += di;
                    }
                }
            }
        }
        return result;
    }
    (x[d - 1]..=m)
        .map(|i| {
            x[d] = i;
            dfs(x, d + 1, n, m, scores)
        })
        .max()
        .unwrap()
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcd: [(Usize1, Usize1, usize, usize); q],
    }
    let mut scores = vec![vec![vec![]; n]; n];
    for &(ai, bi, ci, di) in &abcd {
        scores[ai][bi].push((ci, di));
    }
    let mut ans = vec![1; n];
    println!("{}", dfs(&mut ans, 1, n, m, &scores));
}
