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

fn bfs(graph: &Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let mut distance = vec![INF; graph.len()];
    let mut queue = VecDeque::new();
    distance[s] = 0;
    queue.push_back(s);
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if distance[v] == INF {
                distance[v] = distance[u] + 1;
                queue.push_back(v);
            }
        }
    }
    distance
}

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n1 + n2];
    for &(ai, bi) in ab.iter() {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    let distance_s = bfs(&graph, 0);
    let distance_t = bfs(&graph, n1 + n2 - 1);
    println!(
        "{}",
        (0..n1).map(|i| distance_s[i]).max().unwrap()
            + (n1..n1 + n2).map(|i| distance_t[i]).max().unwrap()
            + 1
    );
}
