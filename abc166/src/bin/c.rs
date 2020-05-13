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
        h: [usize; n],
        ab: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in &ab {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    let mut result = 0;
    for u in 0..n {
        if graph[u].iter().all(|&v| h[u] > h[v]) {
            result += 1;
        }
    }
    println!("{}", result);
}
