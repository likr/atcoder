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
    let mut graph = vec![HashSet::new(); n];
    for &(ai, bi) in &ab {
        graph[ai].insert(bi);
        graph[bi].insert(ai);
    }
    for i in 0..n {
        if graph[i].contains(&0) && graph[i].contains(&(n - 1)) {
            println!("POSSIBLE");
            return;
        }
    }
    println!("IMPOSSIBLE");
}
