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
        q: usize,
        uv: [(Usize1, Usize1); m],
        mut c: [usize; n],
    }
    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: Usize1,
            }
            println!("{}", c[x]);
            for &v in &graph[x] {
                c[v] = c[x];
            }
        } else {
            input! {
                x: Usize1,
                y: usize,
            }
            println!("{}", c[x]);
            c[x] = y;
        }
    }
}
