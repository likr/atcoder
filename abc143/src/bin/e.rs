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
        l: usize,
        abc: [(Usize1, Usize1, usize); m],
        q: usize,
        st: [(Usize1, Usize1); q],
    }
    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let (ai, bi, ci) = abc[i];
        graph[ai].push((bi, ci));
        graph[bi].push((ai, ci));
    }
    let mut distance = vec![vec![(Reverse(INF), 0); n]; n];
    for s in 0..n {
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), l, s));
        distance[s][s] = (Reverse(0), l);
        while let Some((Reverse(d), f, u)) = heap.pop() {
            if distance[s][u] > (Reverse(d), f) {
                continue;
            }
            for &(v, c) in graph[u].iter() {
                if f >= c {
                    if distance[s][v] < (Reverse(d), f - c) {
                        heap.push((Reverse(d), f - c, v));
                        distance[s][v] = (Reverse(d), f - c);
                    }
                } else if l >= c {
                    if distance[s][v] < (Reverse(d + 1), l - c) {
                        heap.push((Reverse(d + 1), l - c, v));
                        distance[s][v] = (Reverse(d + 1), l - c);
                    }
                }
            }
        }
    }
    for i in 0..q {
        let (s, t) = st[i];
        if distance[s][t].0 .0 == INF {
            println!("-1");
        } else {
            println!("{}", distance[s][t].0 .0);
        }
    }
}
