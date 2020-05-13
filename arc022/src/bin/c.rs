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

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in &ab {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }

    let mut h1 = vec![INF; n];
    h1[0] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(0);
    while let Some(u) = queue.pop_front() {
        for &v in &graph[u] {
            if h1[v] == INF {
                h1[v] = h1[u] + 1;
                queue.push_back(v);
            }
        }
    }
    let x = (0..n).max_by_key(|&u| h1[u]).unwrap();

    let mut h2 = vec![INF; n];
    h2[x] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(x);
    while let Some(u) = queue.pop_front() {
        for &v in &graph[u] {
            if h2[v] == INF {
                h2[v] = h2[u] + 1;
                queue.push_back(v);
            }
        }
    }
    let y = (0..n).max_by_key(|&u| h2[u]).unwrap();

    println!("{} {}", x + 1, y + 1);
}
