use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

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
      uv: [(Usize1, Usize1); m],
      s: Usize1,
      t: Usize1,
    }
    let mut graph = vec![vec![]; n * 3];
    for &(u, v) in &uv {
        graph[u].push((n + v, 1));
        graph[n + u].push((2 * n + v, 1));
        graph[2 * n + u].push((v, 1));
    }
    let distance = dijkstra(&graph, s);
    let result = distance[t];
    if result == INF {
        println!("-1");
    } else {
        println!("{}", result / 3);
    }
}
