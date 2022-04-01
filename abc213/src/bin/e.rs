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
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let n = h * w;
    let mut graph = vec![vec![]; n];
    for i in 0..h {
        for j in 1..w {
            let u = i * w + j - 1;
            let v = i * w + j;
            if s[i][j] == '.' {
                graph[u].push((v, 0));
            }
            if s[i][j - 1] == '.' {
                graph[v].push((u, 0));
            }
        }
    }
    for i in 1..h {
        for j in 0..w {
            let u = (i - 1) * w + j;
            let v = i * w + j;
            if s[i][j] == '.' {
                graph[u].push((v, 0));
            }
            if s[i - 1][j] == '.' {
                graph[v].push((u, 0));
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            for di in 0..5 {
                for dj in 0..5 {
                    if (di == 0 && dj == 0)
                        || (di == 0 && dj == 4)
                        || (di == 4 && dj == 0)
                        || (di == 4 && dj == 4)
                    {
                        continue;
                    }
                    if (i + di < 2) || (i + di >= h + 2) || (j + dj < 2) || (j + dj >= w + 2) {
                        continue;
                    }
                    let u = (i + di - 2) * w + j + dj - 2;
                    let v = i * w + j;
                    graph[u].push((v, 1));
                    graph[v].push((u, 1));
                }
            }
        }
    }
    let mut distance = vec![INF; n];
    distance[0] = 0;
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), 0));
    while let Some((Reverse(d), u)) = queue.pop() {
        if distance[u] < d {
            continue;
        }
        for &(v, c) in graph[u].iter() {
            if distance[u] + c < distance[v] {
                distance[v] = distance[u] + c;
                queue.push((Reverse(distance[v]), v));
            }
        }
    }
    println!("{}", distance[n - 1]);
}
