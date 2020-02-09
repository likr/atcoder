use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: i64 = std::i64::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

type Graph = Vec<Vec<(usize, i64)>>;

fn inverse(graph: &Graph) -> Graph {
    let n = graph.len();
    let mut graph2 = vec![vec![]; n];
    for u in 0..n {
        for &(v, c) in &graph[u] {
            graph2[v].push((u, c));
        }
    }
    graph2
}

fn visit(graph: &Graph, u: usize, visited: &mut HashSet<usize>) {
    visited.insert(u);
    for &(v, _) in &graph[u] {
        if !visited.contains(&v) {
            visit(graph, v, visited);
        }
    }
}

fn bellman_ford(graph: &Graph, s: usize, t: usize) -> Option<Vec<i64>> {
    let n = graph.len();
    let mut distance = vec![INF; n];
    let mut parent = vec![None; n];
    distance[s] = 0;
    for _ in 0..n {
        for u in 0..n {
            for &(v, c) in &graph[u] {
                if distance[u] + c < distance[v] {
                    distance[v] = distance[u] + c;
                    parent[v] = Some((u, c));
                }
            }
        }
    }

    let mut v1 = HashSet::new();
    visit(graph, s, &mut v1);
    let inv_graph = inverse(graph);
    let mut v2 = HashSet::new();
    visit(&inv_graph, t, &mut v2);

    for u in 0..n {
        if v1.contains(&u) && v2.contains(&u) {
            for &(v, c) in &graph[u] {
                if distance[u] + c < distance[v] {
                    return None;
                }
            }
        }
    }
    Some(distance)
}

fn main() {
    input! {
      n: usize,
      m: usize,
      p: i64,
      abc: [(Usize1, Usize1, i64); m],
    }
    let mut graph = vec![vec![]; n];
    for &(a, b, c) in &abc {
        graph[a].push((b, p - c));
    }
    if let Some(distance) = bellman_ford(&graph, 0, n - 1) {
        eprintln!("{:?}", distance);
        println!("{}", -min(0, distance[n - 1]));
    } else {
        println!("-1");
    }
}
