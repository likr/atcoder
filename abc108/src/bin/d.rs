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
        mut l: usize,
    }
    let mut r = 1;
    let mut m = 1;
    while 2 * m <= l {
        r += 1;
        m *= 2;
    }
    let mut edges = vec![];
    for i in 1..r {
        edges.push((i - 1, i, 0));
        edges.push((i - 1, i, 2usize.pow(i - 1)));
    }
    for i in (1..r).rev() {
        if l - 2usize.pow(i - 1) >= 2usize.pow(r - 1) {
            edges.push((i - 1, r - 1, l - 2usize.pow(i - 1)));
            l -= 2usize.pow(i - 1);
        }
    }
    println!("{} {}", r, edges.len());
    for &(u, v, d) in &edges {
        println!("{} {} {}", u + 1, v + 1, d);
    }
}
