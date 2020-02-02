use proconio::input;
use std::cmp::min;

const INF: usize = 1000000000000;

fn wf(graph: &Vec<Vec<(usize, usize)>>) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut distance = vec![vec![INF; n]; n];
    for u in 0..n {
        distance[u][u] = 0;
        for &(v, c) in &graph[u] {
            distance[u][v] = c;
            distance[v][u] = c;
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                distance[i][j] = min(distance[i][j], distance[i][k] + distance[k][j]);
            }
        }
    }
    distance
}

fn main() {
    input! {
      n: usize,
      m: usize,
      l: usize,
      abc: [(usize, usize, usize); m],
      q: usize,
      st: [(usize, usize); q],
    }
    let mut graph = vec![vec![]; n + 1];
    for &(ai, bi, ci) in &abc {
        graph[ai].push((bi, ci));
        graph[bi].push((ai, ci));
    }
    let distance = wf(&graph);
    let mut count_graph = vec![vec![]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            if distance[i][j] <= l {
                count_graph[i].push((j, 1));
            }
        }
    }
    let count = wf(&count_graph);
    for &(si, ti) in &st {
        if count[si][ti] == INF {
            println!("-1");
        } else {
            println!("{}", count[si][ti] - 1);
        }
    }
}
