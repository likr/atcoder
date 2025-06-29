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
        h: usize,
        w: usize,
        s0: [Chars; h],
    }
    let mut s = vec![vec!['#'; w + 4]; h + 4];
    for i in 0..h {
        for j in 0..w {
            s[i + 2][j + 2] = s0[i][j];
        }
    }

    let n = (h + 4) * (w + 4);
    let mut graph = vec![vec![]; n];
    for i in 0..h {
        for j in 0..w {
            for di in 0..5 {
                for dj in 0..5 {
                    if s[i + di][j + dj] == '#' && !((di == 0 || di == 4) && (dj == 0 || dj == 4)) {
                        graph[(i + 2) * (w + 4) + (j + 2)].push(((i + di) * (w + 4) + (j + dj), 1));
                    }
                }
            }
        }
    }
    for i in 0..h {
        for j in 1..w {
            if s[i + 2][j + 1] == '.' {
                graph[(i + 2) * (w + 4) + (j + 2)].push(((i + 2) * (w + 4) + (j + 1), 0));
            }
            if s[i + 2][j + 2] == '.' {
                graph[(i + 2) * (w + 4) + (j + 1)].push(((i + 2) * (w + 4) + (j + 2), 0));
            }
        }
    }
    for i in 1..h {
        for j in 0..w {
            if s[i + 1][j + 2] == '.' {
                graph[(i + 2) * (w + 4) + (j + 2)].push(((i + 1) * (w + 4) + (j + 2), 0));
            }
            if s[i + 2][j + 2] == '.' {
                graph[(i + 1) * (w + 4) + (j + 2)].push(((i + 2) * (w + 4) + (j + 2), 0));
            }
        }
    }
    for i in 0..h + 4 {
        debug!(s[i]);
    }
    debug!(s[2][2]);
    debug!(s[h + 1][w + 1]);
    debug!(graph[(h + 1) * (w + 4) + (w)]);
    debug!(graph[(h) * (w + 4) + (w + 1)]);
    let ans = dijkstra(&graph, 2 * (w + 4) + 2);

    println!("{}", ans[(h + 1) * (w + 4) + w + 1]);
}
