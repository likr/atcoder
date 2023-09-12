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
        m: usize,
        a: [[Usize1]; m],
    }
    let mut graph = vec![vec![]; n];
    let mut degree = vec![0; n];
    for i in 0..m {
        for j in 1..a[i].len() {
            graph[a[i][j - 1]].push(a[i][j]);
        }
    }
    for u in 0..n {
        for &v in graph[u].iter() {
            degree[v] += 1;
        }
    }
    let mut queue = VecDeque::new();
    let mut visited = vec![false; n];
    for u in 0..n {
        if degree[u] == 0 {
            queue.push_back(u);
            visited[u] = true;
        }
    }
    let mut ans = vec![];
    while let Some(u) = queue.pop_front() {
        ans.push(u);
        for &v in graph[u].iter() {
            degree[v] -= 1;
            if degree[v] == 0 {
                queue.push_back(v);
                visited[v] = true;
            }
        }
    }
    if ans.len() == n {
        println!("Yes");
    } else {
        println!("No");
    }
}
