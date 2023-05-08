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
    for i in 0..m {
        let (ai, bi, ci, di) = abcd[i];
        graph[ai].push((bi, ci, di));
        graph[bi].push((ai, ci, di));
    }
    let mut distance = vec![INF; n];
    distance[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));
    while let Some((Reverse(d), u)) = heap.pop() {
        if distance[u] < d {
            continue;
        }
        for &(v, a, b) in graph[u].iter() {
            let e = (b as f64).sqrt() as usize + 1;
            let c = if d < e {
                min(a + e - d + b / (e + 1), a + e - 1 - d + b / e)
            } else {
                a + b / (d + 1)
            };
            if d + c < distance[v] {
                distance[v] = d + c;
                heap.push((Reverse(distance[v]), v));
            }
        }
    }
    if distance[n - 1] == INF {
        println!("-1");
    } else {
        println!("{}", distance[n - 1]);
    }
}
