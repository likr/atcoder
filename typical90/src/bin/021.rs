use petgraph::algo::kosaraju_scc;
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
        ab: [(Usize1, Usize1); m],
    }
    let mut graph = Graph::new();
    let indices = (0..n).map(|_| graph.add_node(())).collect::<Vec<_>>();
    for &(ai, bi) in ab.iter() {
        graph.add_edge(indices[ai], indices[bi], ());
    }
    let scc = kosaraju_scc(&graph);
    let mut result = 0;
    for components in scc.iter() {
        let m = components.len();
        result += m * (m - 1) / 2;
    }
    println!("{}", result);
}
