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
        a: [Chars; h],
    }
    let n = h * w;
    let mut graph = vec![vec![]; n + 26];
    for i in 0..h {
        for j in 1..w {
            if a[i][j] != '#' && a[i][j - 1] != '#' {
                let u = i * w + j;
                let v = i * w + j - 1;
                graph[u].push((v, 1));
                graph[v].push((u, 1));
            }
        }
    }
    for i in 1..h {
        for j in 0..w {
            if a[i][j] != '#' && a[i - 1][j] != '#' {
                let u = i * w + j;
                let v = (i - 1) * w + j;
                graph[u].push((v, 1));
                graph[v].push((u, 1));
            }
        }
    }
    let mut s = 0;
    let mut t = 0;
    for i in 0..h {
        for j in 0..w {
            match a[i][j] {
                'S' => {
                    s = i * w + j;
                }
                'G' => {
                    t = i * w + j;
                }
                '.' => {}
                '#' => {}
                _ => {
                    let c = (a[i][j] as u8 - 'a' as u8) as usize;
                    let u = i * w + j;
                    let v = n + c;
                    graph[u].push((v, 1));
                    graph[v].push((u, 0));
                }
            }
        }
    }

    let mut distance = vec![INF; n + 26];
    distance[s] = 0;
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), s));
    while let Some((Reverse(d), u)) = heap.pop() {
        if distance[u] < d {
            continue;
        }
        for &(v, c) in graph[u].iter() {
            if distance[u] + c < distance[v] {
                distance[v] = distance[u] + c;
                heap.push((Reverse(distance[v]), v));
            }
        }
    }
    if distance[t] == INF {
        println!("-1");
    } else {
        println!("{}", distance[t]);
    }
}
