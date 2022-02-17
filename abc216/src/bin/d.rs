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

fn has_cycle(
    u: usize,
    graph: &Vec<Vec<usize>>,
    stack: &mut HashSet<usize>,
    visited: &mut HashSet<usize>,
) -> bool {
    for &v in graph[u].iter() {
        if stack.contains(&v) {
            return true;
        }
        if visited.contains(&v) {
            continue;
        }
        visited.insert(v);
        stack.insert(v);
        if has_cycle(v, graph, stack, visited) {
            return true;
        }
        stack.remove(&v);
    }
    return false;
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let a = (0..m)
        .map(|_| {
            input! {
                k: usize,
                a: [Usize1; k],
            }
            a
        })
        .collect::<Vec<_>>();
    let mut graph = vec![vec![]; n];
    for k in 0..m {
        for i in 1..a[k].len() {
            let u = a[k][i - 1];
            let v = a[k][i];
            graph[u].push(v);
        }
    }
    let mut visited = HashSet::new();
    for s in 0..n {
        if visited.contains(&s) {
            continue;
        }
        let mut stack = HashSet::new();
        visited.insert(s);
        stack.insert(s);
        if has_cycle(s, &graph, &mut stack, &mut visited) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
