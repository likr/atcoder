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
    for i in 0..m {
        let (u, v) = ab[i];
        graph[u].push(v);
    }
    let mut ans = 0;
    for s in 0..n {
        let mut queue = VecDeque::new();
        let mut distance = vec![INF; n];
        queue.push_back(s);
        distance[s] = 0;
        while let Some(u) = queue.pop_front() {
            for &v in graph[u].iter() {
                if distance[v] == INF {
                    queue.push_back(v);
                    distance[v] = distance[u] + 1;
                }
            }
        }
        for u in 0..n {
            if distance[u] != INF {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
