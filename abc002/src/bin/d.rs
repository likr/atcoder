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
        xy: [(Usize1, Usize1); m],
    }
    let mut graph = vec![vec![0; n]; n];
    for &(xi, yi) in &xy {
        graph[xi][yi] = 1;
        graph[yi][xi] = 1;
    }
    let mut result = 0;
    'outer: for x in 1..2usize.pow(n as u32) {
        let vs = (0..n).filter(|&i| x & 1 << i > 0).collect::<Vec<_>>();
        let m = vs.len();
        for i in 0..m {
            for j in 0..i {
                if graph[vs[i]][vs[j]] == 0 {
                    continue 'outer;
                }
            }
        }
        result = max(result, m);
    }
    println!("{}", result);
}
