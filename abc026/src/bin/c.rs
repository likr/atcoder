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

fn solve(graph: &Vec<Vec<usize>>, memo: &mut Vec<usize>, u: usize) -> usize {
    if memo[u] > 0 {
        return memo[u];
    }
    let max_s = if let Some(v) = graph[u].iter().map(|&v| solve(graph, memo, v)).max() {
        v
    } else {
        0
    };
    let min_s = if let Some(v) = graph[u].iter().map(|&v| solve(graph, memo, v)).min() {
        v
    } else {
        0
    };
    memo[u] = max_s + min_s + 1;
    memo[u]
}

fn main() {
    input! {
        n: usize,
        b: [Usize1; n - 1],
    }
    let mut graph = vec![vec![]; n];
    for i in 1..n {
        graph[b[i - 1]].push(i);
    }
    let mut result = vec![0; n];
    println!("{}", solve(&graph, &mut result, 0));
}
