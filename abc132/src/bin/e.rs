use petgraph::algo::dijkstra;
use petgraph::Graph;
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

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
        s: Usize1,
        t: Usize1,
    }
    let mut nodes = HashMap::new();
    let mut graph = Graph::new();
    for i in 0..n {
        for j in 0..3 {
            nodes.insert((i, j), graph.add_node((i, j)));
        }
    }
    for &(ui, vi) in &uv {
        for j in 0..3 {
            graph.add_edge(nodes[&(ui, j)], nodes[&(vi, (j + 1) % 3)], ());
        }
    }
    let distance = dijkstra(&graph, nodes[&(s, 0)], None, |_| 1);
    if let Some(d) = distance.get(&nodes[&(t, 0)]) {
        println!("{}", d / 3);
    } else {
        println!("-1");
    }
}
