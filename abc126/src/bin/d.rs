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
        uvd: [(Usize1, Usize1, usize); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for &(u, v, d) in &uvd {
        graph[u].push((v, d));
        graph[v].push((u, d));
    }

    let mut colors = vec![INF; n];
    let mut queue = VecDeque::new();
    colors[0] = 0;
    for &(v, d) in &graph[0] {
        queue.push_back((0, v, d));
    }
    while let Some((u, v, d)) = queue.pop_front() {
        if d % 2 == 0 {
            colors[v] = colors[u];
        } else {
            colors[v] = 1 - colors[u];
        }
        for &(w, e) in &graph[v] {
            if colors[w] == INF {
                queue.push_back((v, w, e));
            }
        }
    }

    for u in 0..n {
        println!("{}", colors[u]);
    }
}
