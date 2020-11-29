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
        a: Usize1,
        b: Usize1,
        x: usize,
        y: usize,
    }
    let n = 100;
    let mut graph = vec![vec![]; 2 * n];
    for i in 0..n {
        let j = i + n;
        graph[i].push((j, x));
        graph[j].push((i, x));
    }
    for i in 1..n {
        let j = i - 1;
        graph[i].push((j, y));
        graph[j].push((i, y));
        graph[i + n].push((j + n, y));
        graph[j + n].push((i + n, y));
        graph[j + n].push((i, x));
        graph[i].push((j + n, x));
    }

    let distance = dijkstra(&graph, a);
    println!("{}", distance[b + n]);
}
