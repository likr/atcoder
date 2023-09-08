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
        xyp: [(i64, i64, i64); n],
    }
    let mut l = 0;
    let mut h = 8000000000;
    'outer: while h - l > 1 {
        let s = (l + h) / 2;
        let mut graph = vec![vec![]; n];
        for j in 1..n {
            let (xj, yj, pj) = xyp[j];
            for i in 0..j {
                let (xi, yi, pi) = xyp[i];
                if pi * s >= (xi - xj).abs() + (yi - yj).abs() {
                    graph[i].push(j);
                }
                if pj * s >= (xi - xj).abs() + (yi - yj).abs() {
                    graph[j].push(i);
                }
            }
        }
        for i in 0..n {
            let mut distance = vec![INF; n];
            distance[i] = 0;
            let mut queue = VecDeque::new();
            queue.push_back(i);
            while let Some(u) = queue.pop_front() {
                for &v in graph[u].iter() {
                    if distance[v] == INF {
                        distance[v] = distance[u] + 1;
                        queue.push_back(v);
                    }
                }
            }
            if (0..n).all(|i| distance[i] != INF) {
                h = s;
                continue 'outer;
            }
        }
        l = s;
    }
    println!("{}", h);
}
