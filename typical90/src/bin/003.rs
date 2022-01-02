use petgraph::algo::dijkstra;
use petgraph::prelude::*;
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
        ab: [(Usize1, Usize1); n - 1]
    }
    let mut graph = Graph::new_undirected();
    let indices = (0..n).map(|_| graph.add_node(())).collect::<Vec<_>>();
    for &(ai, bi) in ab.iter() {
        graph.add_edge(indices[ai], indices[bi], ());
    }

    let distance = dijkstra(&graph, indices[0], None, |_| 1);
    let farest = (0..n).max_by_key(|&u| distance[&indices[u]]).unwrap();
    let distance = dijkstra(&graph, indices[farest], None, |_| 1);
    let farest = (0..n).max_by_key(|&u| distance[&indices[u]]).unwrap();
    println!("{}", distance[&indices[farest]] + 1)
}
