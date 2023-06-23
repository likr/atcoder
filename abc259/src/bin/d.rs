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

fn dist2(x: i64, y: i64) -> i64 {
    x * x + y * y
}

fn main() {
    input! {
        n: usize,
        s: (i64, i64),
        t: (i64, i64),
        xyr: [(i64, i64, i64); n],
    }
    let mut graph = vec![vec![]; n + 2];
    for i in 0..n {
        let (xi, yi, ri) = xyr[i];
        for j in 0..i {
            let (xj, yj, rj) = xyr[j];
            if dist2(xi - xj, yi - yj) + min(ri, rj) >= max(ri, rj)
                && dist2(xi - xj, yi - yj) <= (ri + rj) * (ri + rj)
            {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
        if dist2(xi - s.0, yi - s.1) == ri * ri {
            graph[i].push(n);
            graph[n].push(i);
        }
        if dist2(xi - t.0, yi - t.1) == ri * ri {
            graph[i].push(n + 1);
            graph[n + 1].push(i);
        }
    }

    let mut d = vec![INF; n + 2];
    d[n] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(n);
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if d[v] == INF {
                d[v] = d[u] + 1;
                queue.push_back(v);
            }
        }
    }
    if d[n + 1] == INF {
        println!("No")
    } else {
        println!("Yes");
    }
}
