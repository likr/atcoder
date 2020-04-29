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
        uvl: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, l) in &uvl {
        graph[u].push((v, l));
        graph[v].push((u, l));
    }
    let mut distance = vec![vec![INF; n]; n];
    for u in 1..n {
        distance[u][u] = 0;
        for &(v, l) in &graph[u] {
            distance[u][v] = l;
        }
    }
    for k in 1..n {
        for i in 1..n {
            for j in 1..n {
                distance[i][j] = min(distance[i][j], distance[i][k] + distance[k][j]);
            }
        }
    }
    let mut result = INF;
    let m = graph[0].len();
    for i in 0..m {
        for j in 0..i {
            let (u, l1) = graph[0][i];
            let (v, l2) = graph[0][j];
            result = min(result, distance[u][v] + l1 + l2);
        }
    }
    if result == INF {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
