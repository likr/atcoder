use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
      s: usize,
      t: usize,
    }
    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
    }
    let mut graph2 = vec![vec![]; n];
    for u in 0..n {
        for &v in &graph[u] {
            graph2[u].push(v);
        }
    }
    let mut graph3 = vec![vec![]; n];
    for u in 0..n {
        for &v in &graph[u] {
            for &w in &graph
            graph2[u].push(v);
        }
    }
    println!("{}", n);
}
