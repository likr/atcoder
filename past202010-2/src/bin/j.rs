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
        x: [usize; 3],
        s: Chars,
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(ai, bi, ci) in &abc {
        graph[ai].push((bi, ci));
        graph[bi].push((ai, ci));
    }

    let distance_s = dijkstra(&graph, 0);
    let distance_t = dijkstra(&graph, n - 1);

    let warp_from_s = "ABC"
        .chars()
        .map(|c| {
            (0..n)
                .filter(|&i| s[i] == c)
                .map(|i| distance_s[i])
                .min()
                .unwrap_or(INF)
        })
        .collect::<Vec<_>>();
    let warp_from_t = "ABC"
        .chars()
        .map(|c| {
            (0..n)
                .filter(|&i| s[i] == c)
                .map(|i| distance_t[i])
                .min()
                .unwrap_or(INF)
        })
        .collect::<Vec<_>>();

    let mut count = vec![0; 3];
    for &c in &s {
        count[c as usize - 'A' as usize] += 1;
    }

    let mut result = distance_s[n - 1];
    result = min(result, warp_from_s[0] + x[0] + warp_from_t[1]);
    result = min(result, warp_from_s[0] + x[1] + warp_from_t[2]);
    result = min(result, warp_from_s[1] + x[0] + warp_from_t[0]);
    result = min(result, warp_from_s[1] + x[2] + warp_from_t[2]);
    result = min(result, warp_from_s[2] + x[1] + warp_from_t[0]);
    result = min(result, warp_from_s[2] + x[2] + warp_from_t[1]);
    if count[0] > 0 {
        result = min(result, warp_from_s[1] + 2 * x[0] + warp_from_t[1]);
        result = min(result, warp_from_s[2] + 2 * x[1] + warp_from_t[2]);
        result = min(result, warp_from_s[1] + x[0] + x[1] + warp_from_t[2]);
        result = min(result, warp_from_s[2] + x[0] + x[1] + warp_from_t[1]);
    }
    if count[1] > 0 {
        result = min(result, warp_from_s[0] + 2 * x[0] + warp_from_t[0]);
        result = min(result, warp_from_s[2] + 2 * x[2] + warp_from_t[2]);
        result = min(result, warp_from_s[0] + x[0] + x[2] + warp_from_t[2]);
        result = min(result, warp_from_s[2] + x[0] + x[2] + warp_from_t[0]);
    }
    if count[2] > 0 {
        result = min(result, warp_from_s[0] + 2 * x[1] + warp_from_t[0]);
        result = min(result, warp_from_s[1] + 2 * x[2] + warp_from_t[1]);
        result = min(result, warp_from_s[0] + x[1] + x[2] + warp_from_t[1]);
        result = min(result, warp_from_s[1] + x[1] + x[2] + warp_from_t[0]);
    }
    if count[0] > 0 && count[1] > 0 && count[2] > 0 {
        result = min(result, warp_from_s[0] + x[0] + x[1] + x[2] + warp_from_t[0]);
        result = min(result, warp_from_s[1] + x[0] + x[1] + x[2] + warp_from_t[1]);
        result = min(result, warp_from_s[2] + x[0] + x[1] + x[2] + warp_from_t[2]);
    }
    println!("{}", result);
}
