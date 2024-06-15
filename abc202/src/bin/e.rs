use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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

fn dfs(
    graph: &Vec<Vec<usize>>,
    u: usize,
    distance: &mut Vec<usize>,
    events: &mut Vec<(usize, usize)>,
) {
    events.push((u, 0));
    for &v in graph[u].iter() {
        if distance[v] == INF {
            distance[v] = distance[u] + 1;
            dfs(graph, v, distance, events);
        }
    }
    events.push((u, 1));
}

fn main() {
    input! {
        n: usize,
        p: [Usize1; n - 1],
        q: usize,
        ud: [(Usize1, usize); q],
    }
    let mut graph = vec![vec![]; n];
    for i in 1..n {
        graph[p[i - 1]].push(i);
        graph[i].push(p[i - 1]);
    }

    let mut distance = vec![INF; n];
    distance[0] = 0;
    let mut events = vec![];
    dfs(&graph, 0, &mut distance, &mut events);
    let mut left = vec![0; n];
    let mut right = vec![0; n];
    for (t, &(u, f)) in events.iter().enumerate() {
        if f == 0 {
            left[u] = t;
        } else {
            right[u] = t;
        }
    }
    let mut index = vec![vec![]; n];
    for u in 0..n {
        index[distance[u]].push(left[u]);
    }
    for i in 0..n {
        index[i].sort();
    }
    for &(ui, di) in ud.iter() {
        println!(
            "{}",
            index[di].upper_bound(&right[ui]) - index[di].lower_bound(&left[ui])
        );
    }
}
