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
const INF: i64 = std::i64::MAX / 4;
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
        h: [i64; n],
        uv: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(ui, vi) in uv.iter() {
        if h[ui] > h[vi] {
            graph[ui].push((vi, (h[vi] - h[ui])));
            graph[vi].push((ui, 2 * (h[ui] - h[vi])));
        } else {
            graph[ui].push((vi, -2 * (h[ui] - h[vi])));
            graph[vi].push((ui, -(h[vi] - h[ui])));
        }
    }

    let mut distance = vec![INF; n];
    distance[0] = 0;
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, 0)));
    while let Some(Reverse((d, u))) = queue.pop() {
        if distance[u] < d {
            continue;
        }
        for &(v, c) in &graph[u] {
            if distance[u] + c < distance[v] {
                distance[v] = distance[u] + c;
                queue.push(Reverse((distance[v], v)));
            }
        }
    }
    println!("{}", -distance.iter().min().unwrap());
}
