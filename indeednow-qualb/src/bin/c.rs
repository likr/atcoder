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
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in &ab {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let mut result = vec![];
    let mut heap = BinaryHeap::new();
    let mut visited = vec![false; n];
    heap.push(Reverse(0));
    while let Some(Reverse(u)) = heap.pop() {
        result.push(format!("{}", u + 1));
        visited[u] = true;
        for &v in &graph[u] {
            if !visited[v] {
                heap.push(Reverse(v));
            }
        }
    }
    println!("{}", result.join(" "));
}
