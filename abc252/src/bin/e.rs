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

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let (ai, bi, ci) = abc[i];
        graph[ai].push((bi, ci));
        graph[bi].push((ai, ci));
    }
    let mut distance = vec![INF; n];
    distance[0] = 0;
    let mut parent = vec![INF; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));
    while let Some((Reverse(d), u)) = heap.pop() {
        if d > distance[u] {
            continue;
        }
        for &(v, c) in graph[u].iter() {
            if distance[u] + c < distance[v] {
                distance[v] = distance[u] + c;
                parent[v] = u;
                heap.push((Reverse(distance[v]), v));
            }
        }
    }
    let mut edges = HashSet::new();
    for u in 1..n {
        edges.insert((u, parent[u]));
        edges.insert((parent[u], u));
    }
    println!(
        "{}",
        (0..m)
            .filter(|&i| edges.contains(&(abc[i].0, abc[i].1)))
            .map(|i| format!("{}", i + 1))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
