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
        ldkcab: [(i64, i64, i64, i64, Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(li, di, ki, ci, ai, bi) in ldkcab.iter() {
        graph[bi].push((ai, li, di, ki, ci));
    }
    let mut distance = vec![-INF; n];
    distance[n - 1] = INF;
    let mut queue = BinaryHeap::new();
    queue.push((distance[n - 1], n - 1));
    while let Some((d, u)) = queue.pop() {
        if distance[u] > d {
            continue;
        }
        for &(v, li, di, ki, ci) in graph[u].iter() {
            let mut ok = 0;
            let mut ng = ki;
            while ng - ok > 1 {
                let m = (ok + ng) / 2;
                let t = li + di * m;
                if t + ci <= d {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            if li + di * ok + ci <= d && li + di * ok > distance[v] {
                distance[v] = li + di * ok;
                queue.push((distance[v], v));
            }
        }
    }
    for u in 0..n - 1 {
        if distance[u] == -INF {
            println!("Unreachable");
        } else {
            println!("{}", distance[u]);
        }
    }
}
