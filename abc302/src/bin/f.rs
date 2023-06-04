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
        m: Usize1,
        mut s: [[Usize1]; n]
    }
    let mut sets = HashMap::new();
    for i in 0..n {
        for &si in s[i].iter() {
            sets.entry(si).or_insert(vec![]).push(i);
        }
    }
    let mut nodes = vec![];
    for &j in sets.keys() {
        for &i in sets[&j].iter() {
            nodes.push((j, i));
        }
        nodes.push((j, INF));
    }
    nodes.push((0, INF));
    nodes.push((m, INF));
    nodes.sort();
    nodes.dedup();
    let node_indices = nodes
        .iter()
        .enumerate()
        .map(|(i, &k)| (k, i))
        .collect::<HashMap<_, _>>();
    let mut graph = vec![vec![]; nodes.len()];
    for i in 0..n {
        for j in 1..s[i].len() {
            graph[node_indices[&(s[i][j - 1], i)]].push((node_indices[&(s[i][j], i)], 0));
            graph[node_indices[&(s[i][j], i)]].push((node_indices[&(s[i][j - 1], i)], 0));
        }
    }
    for &j in sets.keys() {
        for &i in sets[&j].iter() {
            graph[node_indices[&(j, i)]].push((node_indices[&(j, INF)], 1));
            graph[node_indices[&(j, INF)]].push((node_indices[&(j, i)], 0));
        }
    }

    let mut distance = vec![INF; nodes.len()];
    distance[node_indices[&(0, INF)]] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((
        Reverse(distance[node_indices[&(0, INF)]]),
        node_indices[&(0, INF)],
    ));
    while let Some((Reverse(d), u)) = heap.pop() {
        if distance[u] < d {
            continue;
        }
        for &(v, c) in graph[u].iter() {
            if distance[u] + c < distance[v] {
                distance[v] = distance[u] + c;
                heap.push((Reverse(distance[v]), v));
            }
        }
    }
    if distance[node_indices[&(m, INF)]] == INF {
        println!("-1");
    } else {
        println!("{}", distance[node_indices[&(m, INF)]] - 1);
    }
}
