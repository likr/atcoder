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

fn bellman_ford(graph: &Vec<Vec<(usize, isize)>>, root: usize) -> (Vec<isize>, Vec<usize>) {
    let n = graph.len();
    let mut distance = vec![INF as isize; n];
    distance[root] = 0;

    for _ in 1..n {
        for u in 0..n {
            for &(v, c) in &graph[u] {
                if distance[u] + c < distance[v] {
                    distance[v] = distance[u] + c;
                }
            }
        }
    }

    let mut cycle_nodes = HashSet::new();
    for u in 0..n {
        for &(v, c) in &graph[u] {
            if distance[u] + c < distance[v] {
                cycle_nodes.insert(u);
                cycle_nodes.insert(v);
            }
        }
    }
    (distance, cycle_nodes.into_iter().collect::<Vec<_>>())
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
        abc: [(Usize1, Usize1, isize); m],
    }
    let mut graph = vec![vec![]; n];
    let mut graph2 = vec![vec![]; n];
    for &(ai, bi, ci) in &abc {
        graph[ai].push((bi, -ci));
        graph2[ai].push((bi, 1));
    }

    let (distance, cycle_nodes) = bellman_ford(&graph, 0);
    if cycle_nodes.iter().any(|&u| {
        let distance = dijkstra(&graph2, u);
        distance[n - 1] != INF
    }) {
        println!("inf");
    } else {
        println!("{}", -distance[n - 1]);
    }
}
