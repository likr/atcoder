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
        ab: [(usize, usize); n],
    }
    let mut floors = vec![1];
    for i in 0..n {
        floors.push(ab[i].0);
        floors.push(ab[i].1);
    }
    floors.sort();
    floors.dedup();
    let floor_index = floors
        .iter()
        .enumerate()
        .map(|(i, f)| (f, i))
        .collect::<HashMap<_, _>>();
    let mut graph = vec![vec![]; floors.len()];
    for i in 0..n {
        graph[floor_index[&ab[i].0]].push(floor_index[&ab[i].1]);
        graph[floor_index[&ab[i].1]].push(floor_index[&ab[i].0]);
    }
    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut visited = vec![false; floors.len()];
    visited[0] = true;
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if !visited[v] {
                queue.push_back(v);
                visited[v] = true;
            }
        }
    }
    let mut result = 1;
    for i in 0..floors.len() {
        if visited[i] {
            result = max(result, floors[i]);
        }
    }
    println!("{}", result);
}
