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
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            c: [usize; n],
            mut uv: [(Usize1, Usize1); m],
        }
        let mut graph = vec![vec![]; n];
        for i in 0..m {
            let (ui, vi) = uv[i];
            graph[ui].push(vi);
            graph[vi].push(ui);
        }

        let mut s_graph = vec![vec![]; n * n];
        for s in 0..s_graph.len() {
            let u1 = s / n;
            let u2 = s % n;
            for &v1 in graph[u1].iter() {
                for &v2 in graph[u2].iter() {
                    if c[u1] != c[u2] && c[v1] != c[v2] {
                        s_graph[s].push((v1 * n + v2, 1));
                    }
                }
            }
        }

        let d = dijkstra(&s_graph, n - 1);
        debug!(d);

        if d[(n - 1) * n] == INF {
            println!("-1");
        } else {
            println!("{}", d[(n - 1) * n]);
        }
    }
}
