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

fn dfs(graph: &Vec<Vec<usize>>, d: &mut Vec<usize>, u: usize) -> usize {
    if d[u] != 0 {
        return d[u];
    }
    d[u] = 1;
    for &v in &graph[u] {
        if d[v] == 0 {
            d[u] += dfs(graph, d, v);
        }
    }
    d[u]
}

fn main() {
    input! {
        n: usize,
        p: [usize; n - 1],
    }
    let mut graph = vec![vec![]; n];
    for i in 1..n {
        let u = p[i - 1];
        let v = i;
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut d = vec![0; n];
    dfs(&graph, &mut d, 0);
    // eprintln!("{:?}", d);

    for u in 0..n {
        let mut s = n - d[u];
        for &v in &graph[u] {
            if d[v] < d[u] {
                s = max(s, d[v]);
            }
        }
        println!("{}", s);
    }
}
