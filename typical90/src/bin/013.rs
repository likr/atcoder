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
        m: usize,
        abc: [(Usize1, Usize1, i64); m],
    }
    let mut graph = Graph::new_undirected();
    let indices = (0..n).map(|_| graph.add_node(())).collect::<Vec<_>>();
    for &(ai, bi, ci) in abc.iter() {
        graph.add_edge(indices[ai], indices[bi], ci);
    }
    let distance_from_1 = dijkstra(&graph, indices[0], None, |e| *e.weight());
    let distance_from_n = dijkstra(&graph, indices[n - 1], None, |e| *e.weight());
    for i in 0..n {
        println!(
            "{}",
            distance_from_1[&indices[i]] + distance_from_n[&indices[i]]
        );
    }
}
