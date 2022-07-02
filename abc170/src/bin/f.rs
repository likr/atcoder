use ordered_float::*;
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

fn dijkstra(graph: &Vec<Vec<(usize, f64)>>, s: usize) -> Vec<f64> {
    let mut distance = vec![INF as f64; graph.len()];
    distance[s] = 0.;
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((OrderedFloat::from(0f64), s)));
    while !queue.is_empty() {
        let Reverse((d, u)) = queue.pop().unwrap();
        if OrderedFloat::from(distance[u]) < d {
            continue;
        }
        for &(v, c) in &graph[u] {
            if distance[u] + c < distance[v] {
                distance[v] = (distance[u] + c).floor();
                queue.push(Reverse((OrderedFloat::from(distance[v]), v)));
            }
        }
    }
    distance
}

fn main() {
    input! {
        h: usize,
        w: usize,
        k: f64,
        s: (Usize1, Usize1),
        t: (Usize1, Usize1),
        c: [Chars; h],
    }
    let n = h * w * 4 + 2;
    let mut graph = vec![vec![]; n];
    for i in 0..h {
        for j in 1..w {
            for l in 0..4 {
                let u = l * w * h + i * w + j;
                let v = l * w * h + i * w + j - 1;
                graph[u].push((v, 1. / k));
                graph[v].push((u, 1. / k));
            }
        }
    }
    for i in 1..h {
        for j in 0..w {
            for l in 0..4 {
                let u = l * w * h + i * w + j;
                let v = l * w * h + (i - 1) * w + j;
                graph[u].push((v, 1. / k));
                graph[v].push((u, 1. / k));
            }
        }
    }
    for l in 0..4 {
        let u = l * w * h + s.0 * w + s.1;
        let v = n - 2;
        graph[u].push((v, 0.));
        graph[v].push((u, 0.));
        let u = l * w * h + t.0 * w + t.1;
        let v = n - 1;
        graph[u].push((v, 0.));
        graph[v].push((u, 0.));
    }
    let distance = dijkstra(&graph, n - 2);
    println!("{}", distance[n - 1].ceil());
}
