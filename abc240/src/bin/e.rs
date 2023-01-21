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

fn dfs(tree: &Vec<Vec<usize>>, u: usize, l: &mut Vec<usize>, r: &mut Vec<usize>, k: &mut usize) {
    if tree[u].is_empty() {
        l[u] = *k;
        r[u] = *k;
        *k += 1;
    } else {
        for &v in tree[u].iter() {
            dfs(tree, v, l, r, k);
            l[u] = min(l[u], l[v]);
            r[u] = max(r[u], r[v]);
        }
    }
}

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut children = vec![vec![]; n];
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    visited[0] = true;
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if !visited[v] {
                children[u].push(v);
                visited[v] = true;
                queue.push_back(v);
            }
        }
    }

    let mut l = vec![INF; n];
    let mut r = vec![0; n];
    let mut k = 1;
    dfs(&children, 0, &mut l, &mut r, &mut k);
    for i in 0..n {
        println!("{} {}", l[i], r[i]);
    }
}
