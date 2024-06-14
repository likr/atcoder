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
    let mut nodes = vec![1];
    for &(ai, bi) in ab.iter() {
        nodes.push(ai);
        nodes.push(bi);
    }
    nodes.sort();
    nodes.dedup();
    let mut index = HashMap::new();
    for (i, &f) in nodes.iter().enumerate() {
        index.insert(f, i);
    }
    let mut graph = vec![vec![]; nodes.len()];
    for &(ai, bi) in ab.iter() {
        graph[index[&ai]].push(index[&bi]);
        graph[index[&bi]].push(index[&ai]);
    }
    let mut queue = VecDeque::new();
    queue.push_back(index[&1]);
    let mut distance = vec![INF; nodes.len()];
    distance[index[&1]] = 0;
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if distance[v] == INF {
                distance[v] = distance[u] + 1;
                queue.push_back(v);
            }
        }
    }
    let mut ans = 0;
    for &f in nodes.iter() {
        if distance[index[&f]] < INF {
            ans = max(ans, f);
        }
    }
    println!("{}", ans);
}
