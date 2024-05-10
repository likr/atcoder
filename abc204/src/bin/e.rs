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
        abcd: [(Usize1, Usize1, usize, usize); m],
    }
    let mut graph = vec![vec![]; n];
    for &(u, v, c, d) in abcd.iter() {
        if u != v {
            graph[u].push((v, c, d));
            graph[v].push((u, c, d));
        }
    }
    let mut heap = BinaryHeap::new();
    let mut distance = vec![INF; n];
    distance[0] = 0;
    heap.push((Reverse(0), 0));
    while let Some((Reverse(d), u)) = heap.pop() {
        if d > distance[u] {
            continue;
        }
        for &(v, c, d) in graph[u].iter() {
            let cost = if d == 0 {
                c
            } else {
                let t1 = (d as f64).sqrt() as usize - 1;
                if t1 < distance[u] {
                    c + d / (distance[u] + 1)
                } else {
                    let t2 = t1 + 1;
                    min(t1 + d / (t1 + 1) + c, t2 + d / (t2 + 1) + c) - distance[u]
                }
            };
            let new_d = distance[u] + cost;
            if new_d < distance[v] {
                heap.push((Reverse(new_d), v));
                distance[v] = new_d;
            }
        }
    }
    if distance[n - 1] == INF {
        println!("-1");
    } else {
        println!("{}", distance[n - 1]);
    }
}
