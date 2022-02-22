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
        x: Usize1,
        y: Usize1,
    }
    let mut graph = vec![vec![]; n];
    for u in 1..n {
        let v = u - 1;
        graph[u].push(v);
        graph[v].push(u);
    }
    graph[x].push(y);
    graph[y].push(x);
    let mut d = vec![vec![INF; n]; n];
    let mut queue = VecDeque::new();
    for s in 0..n {
        queue.clear();
        queue.push_back(s);
        d[s][s] = 0;
        while let Some(u) = queue.pop_front() {
            for &v in graph[u].iter() {
                if d[s][v] == INF {
                    queue.push_back(v);
                    d[s][v] = d[s][u] + 1;
                }
            }
        }
    }
    let mut count = vec![0; n];
    for i in 0..n {
        for j in 0..i {
            count[d[i][j]] += 1;
        }
    }
    for i in 1..n {
        println!("{}", count[i]);
    }
}
