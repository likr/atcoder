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
        a: usize,
        n: usize,
    }
    let mut graph = vec![vec![]; 1000000];
    for u in 1..graph.len() {
        let v = u * a;
        if v < graph.len() {
            graph[u].push(v);
        }
        let mut x = u;
        let mut base = 1;
        while x >= 10 {
            x /= 10;
            base *= 10;
        }
        if u % 10 != 0 && base > 1 {
            let w = (u % 10) * base + u / 10;
            if w < graph.len() {
                graph[u].push(w);
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(1);
    let mut distance = vec![INF; graph.len()];
    distance[1] = 0;
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if distance[v] == INF {
                queue.push_back(v);
                distance[v] = distance[u] + 1;
            }
        }
    }
    if distance[n] == INF {
        println!("-1");
    } else {
        println!("{}", distance[n]);
    }
}
