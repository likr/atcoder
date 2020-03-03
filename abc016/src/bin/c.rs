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
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![]; n];
    for &(ai, bi) in &ab {
        graph[ai].push(bi);
        graph[bi].push(ai);
    }
    for u in 0..n {
        let mut friend = graph[u].iter().map(|&v| v).collect::<HashSet<_>>();
        friend.insert(u);
        let mut friend_friend = HashSet::new();
        for &v in &graph[u] {
            for &w in &graph[v] {
                if !friend.contains(&w) {
                    friend_friend.insert(w);
                }
            }
        }
        println!("{}", friend_friend.len());
    }
}
