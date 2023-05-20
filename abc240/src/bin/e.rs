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

fn rec(children: &Vec<Vec<usize>>, l: &mut Vec<usize>, r: &mut Vec<usize>, u: usize) {
    for &v in children[u].iter() {
        rec(children, l, r, v);
        l[u] = min(l[u], l[v]);
        r[u] = max(r[u], r[v]);
    }
}

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for &(ui, vi) in uv.iter() {
        graph[ui].push(vi);
        graph[vi].push(ui);
    }
    let mut children = vec![vec![]; n];
    let mut stack = vec![];
    stack.push(0);
    let mut visited = vec![false; n];
    visited[0] = true;
    while let Some(u) = stack.pop() {
        for &v in graph[u].iter() {
            if !visited[v] {
                children[u].push(v);
                stack.push(v);
                visited[v] = true;
            }
        }
    }
    let mut count = 1;
    let mut l = vec![INF; n];
    let mut r = vec![0; n];
    let mut stack = vec![];
    stack.push(0);
    let mut visited = vec![false; n];
    visited[0] = true;
    while let Some(u) = stack.pop() {
        for &v in graph[u].iter() {
            if !visited[v] {
                if children[v].len() == 0 {
                    l[v] = count;
                    r[v] = count;
                    count += 1;
                }
                stack.push(v);
                visited[v] = true;
            }
        }
    }
    rec(&children, &mut l, &mut r, 0);
    for i in 0..n {
        println!("{} {}", l[i], r[i]);
    }
}
