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
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), s));
    while let Some((Reverse(d), u)) = heap.pop() {
        if distance[u] < d {
            continue;
        }
        for &(v, c) in graph[u].iter() {
            if distance[u] + c < distance[v] {
                distance[v] = distance[u] + c;
                heap.push((Reverse(distance[v]), v));
            }
        }
    }
    distance
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut graph = vec![vec![]; n + 1];
    for i in 0..m {
        let (ui, vi) = uv[i];
        graph[ui].push((vi, 1));
        graph[vi].push((ui, 1));
    }
    let distance1 = dijkstra(&graph, 1);
    let distancen = dijkstra(&graph, n);
    let mut result = vec![];
    for i in 1..=n {
        let d = min(
            distance1[n],
            min(distance1[0] + distancen[i], distance1[i] + distancen[0]),
        );

        if d >= INF {
            result.push(format!("-1"));
        } else {
            result.push(format!("{}", d));
        }
    }
    println!("{}", result.join(" "));
}
