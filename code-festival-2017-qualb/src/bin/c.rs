use petgraph::algo::dijkstra;
use petgraph::graph::*;
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
        ab: [(Usize1, Usize1); m],
    }
    let mut graph = Graph::new_undirected();
    for _ in 0..n {
        graph.add_node(());
    }
    for &(ai, bi) in &ab {
        graph.add_edge(node_index(ai), node_index(bi), ());
    }
    let distance = dijkstra(&graph, node_index(0), None, |_| 1);
    let mut color = vec![0; n];
    for i in 0..n {
        if distance[&node_index(i)] % 2 == 0 {
            color[i] = 1;
        }
    }
    if ab.iter().any(|&(ai, bi)| color[ai] == color[bi]) {
        println!("{}", n * (n - 1) / 2 - m);
    } else {
        let mut count = [0, 0];
        for i in 0..n {
            count[color[i]] += 1;
        }
        println!("{}", count[0] * count[1] - m);
    }
}
