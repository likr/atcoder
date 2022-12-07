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
        k: usize,
        uva: [(Usize1, Usize1, usize); m],
        s: [Usize1; k],
    }
    let mut graph = vec![vec![]; 2 * n];
    for &(ui, vi, ai) in uva.iter() {
        if ai == 1 {
            graph[ui].push((vi, 1));
            graph[vi].push((ui, 1));
        } else {
            graph[n + ui].push((n + vi, 1));
            graph[n + vi].push((n + ui, 1));
        }
    }
    for &si in s.iter() {
        graph[si].push((si + n, 0));
        graph[si + n].push((si, 0));
    }

    let distance = dijkstra(&graph, 0);
    let result = min(distance[n - 1], distance[2 * n - 1]);
    if result == INF {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
