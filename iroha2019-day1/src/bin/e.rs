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
        a: usize,
        b: usize,
        mut d: [usize; b],
    }
    d.push(0);
    d.sort();
    let mut s = 0;
    for i in 1..d.len() {
        let e = d[i] - d[i - 1] - 1;
        s += e - e / a
    }
    let e = n - d[d.len() - 1];
    s += e - e / a;
    println!("{}", s);
}
