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

fn dfs(
    graph: &Vec<Vec<usize>>,
    u: usize,
    p: Option<usize>,
    visited: &mut Vec<bool>,
    lamp: &mut Vec<bool>,
    ans: &mut Vec<(usize, usize)>,
) {
    for &v in graph[u].iter() {
        if !visited[v] {
            visited[v] = true;
            dfs(graph, v, Some(u), visited, lamp, ans);
        }
    }
    if let Some(p) = p {
        if !lamp[u] {
            lamp[u] = true;
            lamp[p] = !lamp[p];
            ans.push((p, u));
        }
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uv: [(Usize1, Usize1); m],
    }
    if k % 2 == 1 {
        println!("No");
        return;
    }
    let mut graph = vec![vec![]; n];
    let mut edges = HashMap::new();
    for (i, &(ui, vi)) in uv.iter().enumerate() {
        graph[ui].push(vi);
        graph[vi].push(ui);
        edges.insert((ui, vi), i);
        edges.insert((vi, ui), i);
    }
    let mut visited = vec![false; n];
    let mut lamp = vec![false; n];
    let mut ans = vec![];
    for u in 0..n {
        if !visited[u] {
            visited[u] = true;
            dfs(&graph, u, None, &mut visited, &mut lamp, &mut ans);
        }
    }
    let mut ans_edges = vec![];
    let mut lamp = vec![false; n];
    let mut count = 0;
    for &(u, v) in ans.iter() {
        if count == k {
            break;
        }
        ans_edges.push(format!("{}", edges[&(u, v)] + 1));
        lamp[u] = !lamp[u];
        lamp[v] = !lamp[v];
        if lamp[u] {
            count += 1;
        } else {
            count -= 1;
        }
        if lamp[v] {
            count += 1;
        } else {
            count -= 1;
        }
    }
    if count == k {
        println!("Yes");
        println!("{}", ans_edges.len());
        println!("{}", ans_edges.join(" "));
    } else {
        println!("No");
    }
}
