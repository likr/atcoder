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

fn height(graph: &Vec<Vec<usize>>, h: &mut Vec<usize>, u: usize) -> usize {
    if h[u] != INF {
        return h[u];
    }
    h[u] = graph[u]
        .iter()
        .map(|&v| height(graph, h, v) + 1)
        .max()
        .unwrap();
    h[u]
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n + m - 1],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in &ab {
        graph[bi].push(ai);
    }

    let mut root = 0;
    for u in 0..n {
        if graph[u].len() == 0 {
            root = u;
        }
    }
    eprintln!("{}", root);

    let mut h = vec![INF; n];
    h[root] = 0;
    for u in 0..n {
        height(&graph, &mut h, u);
    }

    let mut parent = vec![0; n];
    for u in 0..n {
        for &v in &graph[u] {
            if h[u] - h[v] == 1 {
                parent[u] = v + 1;
            }
        }
    }
    for u in 0..n {
        println!("{}", parent[u]);
    }
}
