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
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(ai, bi) in ab.iter() {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let mut distance = vec![INF; graph.len()];
    distance[0] = 0;
    let mut dp = vec![0; n];
    dp[0] = 1;
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), 0));
    while let Some((Reverse(d), u)) = queue.pop() {
        if distance[u] < d {
            continue;
        }
        for &v in &graph[u] {
            if distance[u] + 1 == distance[v] {
                dp[v] = (dp[v] + dp[u]) % M;
            }
            if distance[u] + 1 < distance[v] {
                distance[v] = distance[u] + 1;
                dp[v] = dp[u];
                queue.push((Reverse(distance[v]), v));
            }
        }
    }
    println!("{}", dp[n - 1]);
}
