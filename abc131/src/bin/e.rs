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
        k: usize,
    }
    let m = (n - 1) * (n - 2) / 2;
    if k > m {
        println!("-1");
        return;
    }
    let mut edges = vec![];
    for i in 1..n {
        edges.push((0, i));
    }
    'outer: for i in 2..n {
        for j in 1..i {
            if edges.len() == n * (n - 1) / 2 - k {
                break 'outer;
            }
            edges.push((i, j));
        }
    }
    println!("{}", edges.len());
    for &(i, j) in &edges {
        println!("{} {}", i + 1, j + 1);
    }
}
