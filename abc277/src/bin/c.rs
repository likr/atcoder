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
        let (ai, bi) = ab[i];
        floors.push(ai);
        floors.push(bi);
    }
    floors.sort();
    floors.dedup();
    let floor_index = floors
        .iter()
        .enumerate()
        .map(|(i, &f)| (f, i))
        .collect::<HashMap<_, _>>();
    let mut graph = vec![vec![]; floors.len()];
    for i in 0..n {
        let (ai, bi) = ab[i];
        graph[floor_index[&ai]].push(floor_index[&bi]);
        graph[floor_index[&bi]].push(floor_index[&ai]);
    }
    let mut distance = vec![INF; floors.len()];
    distance[0] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        for &v in graph[u].iter() {
            if distance[v] == INF {
                distance[v] = distance[u] + 1;
                queue.push_back(v);
            }
        }
    }
    let mut result = 0;
    for i in 0..floors.len() {
        if distance[i] != INF {
            result = i;
        }
    }
    println!("{}", floors[result]);
}
