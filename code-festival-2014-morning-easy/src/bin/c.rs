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
        s: Usize1,
        t: Usize1,
        xyd: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(xi, yi, di) in &xyd {
        graph[xi].push((yi, di));
        graph[yi].push((xi, di));
    }
    let mut candidate = vec![];
    for u in 0..n {
        let distance = dijkstra(&graph, u);
        if distance[s] != INF && distance[s] == distance[t] {
            candidate.push((distance[s], u + 1));
        }
    }
    if candidate.is_empty() {
        println!("-1");
    } else {
        candidate.sort();
        println!("{}", candidate[0].1);
    }
}
