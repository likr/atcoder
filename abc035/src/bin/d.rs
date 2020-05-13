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
        t: usize,
        a: [usize; n],
        abc: [(Usize1, Usize1, usize); m],
    }
    let mut graph = vec![vec![]; n];
    let mut inv_graph = vec![vec![]; n];
    for &(ai, bi, ci) in &abc {
        graph[ai].push((bi, ci));
        inv_graph[bi].push((ai, ci));
    }

    let d = dijkstra(&graph, 0);
    let e = dijkstra(&inv_graph, 0);

    let mut result = 0;
    for u in 0..n {
        if d[u] + e[u] <= t {
            result = max(result, a[u] * (t - d[u] - e[u]));
        }
    }
    println!("{}", result);
}
