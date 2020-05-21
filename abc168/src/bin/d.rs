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

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<(usize, usize)> {
    let mut distance = vec![(INF, INF); graph.len()];
    distance[s] = (0, 0);
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, s)));
    while !queue.is_empty() {
        let Reverse((d, u)) = queue.pop().unwrap();
        if distance[u].0 < d {
            continue;
        }
        for &(v, c) in &graph[u] {
            if distance[u].0 + c < distance[v].0 {
                distance[v] = (distance[u].0 + c, u);
                queue.push(Reverse((distance[v].0, v)));
            }
        }
    }
    distance
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in &ab {
        graph[ai].push((bi, 1));
        graph[bi].push((ai, 1));
    }
    let distance = dijkstra(&graph, 0);
    println!("Yes");
    for i in 1..n {
        println!("{}", distance[i].1 + 1);
    }
}
