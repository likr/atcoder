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
        st: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for k in 0..m {
        let (u, v) = st[k];
        graph[u].push((v, k));
    }
    let mut distance = vec![INF; n];
    distance[0] = 0;
    let mut parent = vec![None; n];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        for &(v, _) in graph[u].iter() {
            if distance[v] == INF {
                distance[v] = distance[u] + 1;
                parent[v] = Some(u);
                queue.push_back(v);
            }
        }
    }
    let mut min_edges = HashSet::new();
    let mut v = n - 1;
    while let Some(u) = parent[v] {
        min_edges.insert((u, v));
        v = u;
    }
    for k in 0..m {
        if min_edges.contains(&st[k]) {
            let mut distance = vec![INF; n];
            distance[0] = 0;
            let mut queue = VecDeque::new();
            queue.push_back(0);
            while let Some(u) = queue.pop_front() {
                for &(v, l) in graph[u].iter() {
                    if k == l {
                        continue;
                    }
                    if distance[v] == INF {
                        distance[v] = distance[u] + 1;
                        queue.push_back(v);
                    }
                }
            }
            if distance[n - 1] == INF {
                println!("-1");
            } else {
                println!("{}", distance[n - 1]);
            }
        } else {
            if distance[n - 1] == INF {
                println!("-1");
            } else {
                println!("{}", distance[n - 1]);
            }
        }
    }
}
