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

fn warshall_floyd(graph: &Vec<Vec<(usize, usize)>>) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut d = vec![vec![INF; n]; n];
    for u in 0..n {
        d[u][u] = 0;
        for &(v, c) in &graph[u] {
            d[u][v] = c;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abt: [(Usize1, Usize1, usize); m],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi, ti) in &abt {
        graph[ai].push((bi, ti));
        graph[bi].push((ai, ti));
    }
    let d = warshall_floyd(&graph);
    println!(
        "{}",
        d.iter().map(|row| row.iter().max().unwrap()).min().unwrap()
    );
}
