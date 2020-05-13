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
        abc: [(Usize1, Usize1, usize); m],
        k: usize,
        xyz: [(Usize1, Usize1, usize); k],
    }
    let mut distance = vec![vec![INF; n]; n];
    for i in 0..n {
        distance[i][i] = 0;
    }
    for &(ai, bi, ci) in &abc {
        distance[ai][bi] = ci;
        distance[bi][ai] = ci;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                distance[i][j] = min(distance[i][j], distance[i][k] + distance[k][j]);
            }
        }
    }
    for &(xi, yi, zi) in &xyz {
        let mut s = 0;
        for i in 0..n {
            for j in 0..n {
                let d = min(
                    distance[i][xi] + zi + distance[yi][j],
                    distance[i][yi] + zi + distance[xi][j],
                );
                distance[i][j] = min(distance[i][j], d);
                s += distance[i][j];
            }
        }
        println!("{}", s / 2);
    }
}
