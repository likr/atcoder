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
        a: [usize; n],
        q: usize,
        bc: [(usize, usize); q],
    }
    let mut d = vec![0usize; 100001];
    for &ai in &a {
        d[ai] += 1;
    }
    // eprintln!("{:?}", d);
    let mut s = 0;
    for i in 0..d.len() {
        s += i * d[i];
    }
    for &(b, c) in &bc {
        let k = d[b];
        s -= k * b;
        s += k * c;
        d[b] = 0;
        d[c] += k;
        println!("{}", s);
    }
}
