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
        q: usize,
        ftx: [(Usize1, Usize1, Usize1); q],
    }
    let mut parent = (0..n).map(|i| i + n).collect::<Vec<_>>();
    let mut tail = (0..n).map(|i| Some(i)).collect::<Vec<_>>();
    for i in 0..q {
        let (fi, ti, xi) = ftx[i];
        let pxi = parent[xi];
        if let Some(t) = tail[ti] {
            parent[xi] = t;
        } else {
            parent[xi] = ti + n;
        }
        if pxi >= n {
            tail[ti] = tail[fi];
            tail[fi] = None;
        } else {
            tail[ti] = tail[fi];
            tail[fi] = Some(pxi);
        }
        // eprintln!("{}", i);
        // eprintln!("{:?}", parent);
        // eprintln!("{:?}", tail);
    }
    let mut result = vec![0; n];
    for i in 0..n {
        if let Some(mut t) = tail[i] {
            loop {
                result[t] = i + 1;
                if parent[t] >= n {
                    break;
                }
                t = parent[t];
            }
        }
    }
    for i in 0..n {
        println!("{}", result[i]);
    }
}
