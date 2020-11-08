use ordered_float::OrderedFloat;
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
    queue.push(Reverse((OrderedFloat::from(0.), s)));
    while !queue.is_empty() {
        let Reverse((d, u)) = queue.pop().unwrap();
        if OrderedFloat::from(distance[u]) < d {
            continue;
        }
        for &(v, c) in &graph[u] {
            if distance[u] + c < distance[v] {
                distance[v] = distance[u] + c;
                queue.push(Reverse((OrderedFloat::from(distance[v]), v)));
            }
        }
    }
    distance
}

fn main() {
    input! {
        s: (f64, f64),
        t: (f64, f64),
        n: usize,
        mut xyr: [(f64, f64, f64); n],
    }

    xyr.push((s.0, s.1, 0.));
    xyr.push((t.0, t.1, 0.));
    let m = n + 2;

    let mut graph = vec![vec![]; m];
    for i in 0..m {
        let (xi, yi, ri) = xyr[i];
        for j in 0..m {
            let (xj, yj, rj) = xyr[j];
            let dx = xi - xj;
            let dy = yi - yj;
            let d = (dx * dx + dy * dy).sqrt();
            graph[i].push((j, if d <= ri + rj { 0. } else { d - ri - rj }));
        }
    }

    let distance = dijkstra(&graph, n);
    println!("{}", distance[n + 1]);
}
