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
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    let mut graph = Graph::new();
    let mut nodes = HashMap::new();
    let mut s = (INF, INF);
    let mut g = (INF, INF);
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 's' {
                s = (i, j);
            }
            if c[i][j] == 'g' {
                g = (i, j);
            }
            nodes.insert((i, j), graph.add_node(()));
        }
    }
    for i in 0..h {
        for j in 1..w {
            if c[i][j] == '#' {
                graph.add_edge(nodes[&(i, j - 1)], nodes[&(i, j)], 1);
            } else {
                graph.add_edge(nodes[&(i, j - 1)], nodes[&(i, j)], 0);
            }
            if c[i][j - 1] == '#' {
                graph.add_edge(nodes[&(i, j)], nodes[&(i, j - 1)], 1);
            } else {
                graph.add_edge(nodes[&(i, j)], nodes[&(i, j - 1)], 0);
            }
        }
    }
    for i in 1..h {
        for j in 0..w {
            if c[i][j] == '#' {
                graph.add_edge(nodes[&(i - 1, j)], nodes[&(i, j)], 1);
            } else {
                graph.add_edge(nodes[&(i - 1, j)], nodes[&(i, j)], 0);
            }
            if c[i - 1][j] == '#' {
                graph.add_edge(nodes[&(i, j)], nodes[&(i - 1, j)], 1);
            } else {
                graph.add_edge(nodes[&(i, j)], nodes[&(i - 1, j)], 0);
            }
        }
    }
    let distance = dijkstra(&graph, nodes[&s], Some(nodes[&g]), |e| *e.weight());
    if distance[&nodes[&g]] <= 2 {
        println!("YES");
    } else {
        println!("NO");
    }
}
