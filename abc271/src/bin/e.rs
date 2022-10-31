use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

#[allow(unused_macros)]
macro_rules! debug {
    ($($a:expr),* $(,)*) => {
        #[cfg(debug_assertions)]
        eprintln!(concat!($("| ", stringify!($a), "={:?} "),*, "|"), $(&$a),*);
    };
}

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
    let mut distance = vec![INF; graph.len()];
    distance[s] = 0;
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, s)));
    while !queue.is_empty() {
        let Reverse((d, u)) = queue.pop().unwrap();
        if distance[u] < d {
            continue;
        }
        for &(v, c) in &graph[u] {
            if distance[u] + c < distance[v] {
                distance[v] = distance[u] + c;
                queue.push(Reverse((distance[v], v)));
            }
        }
    }
    distance
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        abc: [(Usize1, Usize1, usize); m],
        e: [Usize1; k],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let (ai, bi, ci) = abc[i];
        graph[ai].push((bi, ci, i));
    }
    let mut u = vec![0];
    for i in 0..k {
        u.push(abc[e[i]].1);
    }
    // debug!(u);
    let mut edge_indices = vec![vec![]; m];
    for i in 0..k {
        edge_indices[e[i]].push(i);
    }
    let mut graph2 = vec![vec![]; k + 1];
    for i in 0..k {
        for &(_v, ci, ei) in graph[u[i]].iter() {
            let j = edge_indices[ei].lower_bound(&i);
            if j < edge_indices[ei].len() {
                graph2[i].push((edge_indices[ei][j] + 1, ci));
            }
        }
    }
    // debug!(graph2);
    // let distance = dijkstra(&graph2, 0);
    // // debug!(distance);
    // let mut result = INF;
    // for i in 1..=k {
    //     if u[i] == n - 1 {
    //         result = min(result, distance[i]);
    //     }
    // }
    // if result == INF {
    //     println!("-1");
    // } else {
    //     println!("{}", result);
    // }
}
