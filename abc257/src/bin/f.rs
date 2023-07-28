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
        uv: [(usize, usize); m],
    }
    let mut graph = vec![vec![]; n + 1];
    for &(u, v) in uv.iter() {
        graph[u].push((v, 1));
        graph[v].push((u, 1));
    }
    let distance_1 = dijkstra(&graph, 1);
    let distance_n = dijkstra(&graph, n);
    let mut result = vec![];
    for i in 1..=n {
        let d = [
            distance_1[n],
            distance_1[i] + distance_n[0],
            distance_1[0] + distance_n[i],
        ];
        let &min_d = d.iter().min().unwrap();
        if min_d == INF {
            result.push(format!("-1"));
        } else {
            result.push(format!("{}", min_d));
        }
    }
    println!("{}", result.join(" "));
}
