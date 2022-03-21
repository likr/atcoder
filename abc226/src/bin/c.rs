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
    }
    let mut t = vec![];
    let mut graph = vec![vec![]; n];
    for u in 0..n {
        input! {
            ti: usize,
            ki: usize,
            ai: [Usize1; ki],
        }
        t.push(ti);
        for &v in ai.iter() {
            graph[u].push(v);
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(n - 1);
    let mut visited = HashSet::new();
    visited.insert(n - 1);
    let mut result = t[n - 1];
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if !visited.contains(&v) {
                queue.push_back(v);
                visited.insert(v);
                result += t[v];
            }
        }
    }
    println!("{}", result);
}
