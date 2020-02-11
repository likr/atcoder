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

fn longest_path(graph: &Vec<Vec<usize>>, s: usize, length: &mut Vec<usize>) -> usize {
    if graph[s].len() == 0 {
        length[s] = 0;
        return 0;
    }
    if length[s] > 0 {
        return length[s];
    }
    for &v in &graph[s] {
        length[s] = max(length[s], longest_path(graph, v, length) + 1);
    }
    length[s]
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(xi, yi) in &xy {
        graph[xi].push(yi);
    }
    let mut length = vec![0; n];
    for u in 0..n {
        longest_path(&graph, u, &mut length);
    }
    println!("{}", length.iter().max().unwrap());
}
