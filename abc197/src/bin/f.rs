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

fn dijkstra(graph: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
    let mut distance = vec![INF; graph.len()];
    distance[s] = 0;
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, s)));
    while !queue.is_empty() {
        let Reverse((d, u)) = queue.pop().unwrap();
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
    distance
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, char); m],
    }
    let mut graph = vec![vec![]; n * n];
    for j in 0..m {
        for i in 0..j {
            let (ai, bi, ci) = abc[i];
            let (aj, bj, cj) = abc[j];
            if ci == cj {
                graph[ai * n + aj].push((bi * n + bj, 1));
                graph[bi * n + bj].push((ai * n + aj, 1));
                graph[ai * n + bj].push((bi * n + aj, 1));
                graph[bi * n + aj].push((ai * n + bj, 1));
                graph[aj * n + ai].push((bj * n + bi, 1));
                graph[aj * n + bi].push((bj * n + ai, 1));
                graph[bj * n + ai].push((aj * n + bi, 1));
                graph[bj * n + bi].push((aj * n + ai, 1));
            }
        }
    }
    let dist = dijkstra(&graph, n - 1);
    let mut ans = INF;
    for i in 0..n {
        ans = min(ans, dist[i * n + i] * 2);
    }
    for i in 0..m {
        let (ai, bi, _) = abc[i];
        ans = min(ans, dist[ai * n + bi] * 2 + 1);
        ans = min(ans, dist[bi * n + ai] * 2 + 1);
    }
    if ans < INF {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
