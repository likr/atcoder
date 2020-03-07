use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use superslice::*;

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
    let mut graph = vec![vec![false; n]; n];
    for &(ai, bi) in &ab {
        graph[ai][bi] = true;
        graph[bi][ai] = true;
    }
    let mut indices = (1..n).collect::<Vec<_>>();
    let mut count = 0;
    loop {
        if graph[0][indices[0]] && (1..n - 1).all(|i| graph[indices[i - 1]][indices[i]]) {
            count += 1;
        }
        if !indices.next_permutation() {
            break;
        }
    }
    println!("{}", count);
}
