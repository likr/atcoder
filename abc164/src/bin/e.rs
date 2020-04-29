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
        s: usize,
        uvab: [(Usize1, Usize1, usize, usize); m],
        cd: [(usize, usize); n],
    }

    let mut graph = vec![vec![]; n];
    for &(ui, vi, ai, bi) in &uvab {
        graph[ui].push((vi, ai, bi));
        graph[vi].push((ui, ai, bi));
    }

    let m = n * 50;
    let mut distance = vec![vec![INF; m + 1]; n];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, min(s, m), 0)));

    while let Some(Reverse((d, c, u))) = heap.pop() {
        if distance[u][c] != INF {
            continue;
        }
        distance[u][c] = d;
        let (cu, du) = cd[u];
        if c + cu <= m {
            heap.push(Reverse((d + du, c + cu, u)));
        }
        for &(v, a, b) in &graph[u] {
            if a <= c {
                heap.push(Reverse((d + b, c - a, v)));
            }
        }
    }

    for u in 1..n {
        println!("{}", distance[u].iter().min().unwrap());
    }
}
