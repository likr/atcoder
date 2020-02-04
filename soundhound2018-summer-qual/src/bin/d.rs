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
      s: Usize1,
      t: Usize1,
      uvab: [(Usize1, Usize1, usize, usize); m],
    }

    let mut graph_a = vec![vec![]; n];
    let mut graph_b = vec![vec![]; n];
    for &(u, v, ai, bi) in &uvab {
        graph_a[u].push((v, ai));
        graph_a[v].push((u, ai));
        graph_b[u].push((v, bi));
        graph_b[v].push((u, bi));
    }

    let initial = 1000000000000000;
    let d1 = dijkstra(&graph_a, s);
    let d2 = dijkstra(&graph_b, t);
    // println!("{:?}", d1);
    // println!("{:?}", d2);
    let mut result = vec![0; n];
    let mut heap = BinaryHeap::new();
    for i in (0..n).rev() {
        // println!("{} {}", d1[i], d2[i]);
        heap.push(Reverse(d1[i] + d2[i]));
        let Reverse(r) = heap.peek().unwrap();
        result[i] = initial - *r;
    }
    for &r in &result {
        println!("{}", r);
    }
}
