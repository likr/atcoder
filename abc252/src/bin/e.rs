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
    let mut parent = vec![INF; graph.len()];
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
                parent[v] = u;
                queue.push(Reverse((distance[v], v)));
            }
        }
    }
    parent
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi, ci) in abc.iter() {
        graph[ai].push((bi, ci));
        graph[bi].push((ai, ci));
    }
    let parent = dijkstra(&graph, 0);
    let mut edges = HashSet::new();
    for u in 1..n {
        let v = parent[u];
        edges.insert((u, v));
        edges.insert((v, u));
    }
    let mut result = vec![];
    for i in 0..m {
        let (u, v, _) = abc[i];
        if edges.contains(&(u, v)) || edges.contains(&(v, u)) {
            result.push(format!("{}", i + 1));
        }
    }
    println!("{}", result.join(" "));
}
