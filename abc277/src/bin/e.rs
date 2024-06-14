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
    let mut graph = vec![vec![]; n * 2];
    for &(ui, vi, ai) in uva.iter() {
        if ai == 1 {
            graph[ui].push((vi, 1));
            graph[vi].push((ui, 1));
        } else {
            graph[ui + n].push((vi + n, 1));
            graph[vi + n].push((ui + n, 1));
        }
    }
    for &u in s.iter() {
        graph[u].push((u + n, 0));
        graph[u + n].push((u, 0));
    }
    let distance = dijkstra(&graph, 0);
    let ans = min(distance[n - 1], distance[2 * n - 1]);
    if ans == INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
