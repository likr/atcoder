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
    queue.push(Reverse((distance[s], s)));
    while let Some(Reverse((d, u))) = queue.pop() {
        if d > distance[u] {
            continue;
        }
        for &(v, c) in &graph[u] {
            if d + c < distance[v] {
                distance[v] = d + c;
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
        cab: [(usize, usize, usize); m],
    }

    let m = 100;
    let mut graph = vec![vec![]; n * m];
    for &(ci, ai, bi) in &cab {
        if ci == 0 {
            for j in 0..m {
                graph[j * n + ai].push((j * n + bi, 1));
                graph[j * n + bi].push((j * n + ai, 1));
            }
        } else {
            for j in 1..m {
                graph[(j - 1) * n + ai].push((j * n + bi, j));
                graph[(j - 1) * n + bi].push((j * n + ai, j));
            }
        }
    }

    let distance = dijkstra(&graph, 0);
    for i in 0..n {
        println!("{}", (0..m).map(|j| distance[j * n + i]).min().unwrap());
    }
}
