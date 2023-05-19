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
        k: usize,
        uva: [(Usize1, Usize1, usize); m],
        s: [Usize1; k],
    }
    let mut graph = vec![vec![]; n * 2];
    for i in 0..m {
        let (ui, vi, ai) = uva[i];
        if ai == 1 {
            graph[ui].push((vi, 1));
            graph[vi].push((ui, 1));
        } else {
            graph[ui + n].push((vi + n, 1));
            graph[vi + n].push((ui + n, 1));
        }
    }
    for i in 0..k {
        let si = s[i];
        graph[si].push((si + n, 0));
        graph[si + n].push((si, 0));
    }
    let mut distance = vec![INF; n * 2];
    distance[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(distance[0]), 0));
    while let Some((Reverse(d), u)) = heap.pop() {
        if distance[u] < d {
            continue;
        }
        for &(v, c) in graph[u].iter() {
            if distance[u] + c < distance[v] {
                distance[v] = distance[u] + c;
                heap.push((Reverse(distance[v]), v));
            }
        }
    }
    let result = min(distance[n - 1], distance[2 * n - 1]);
    if result == INF {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
