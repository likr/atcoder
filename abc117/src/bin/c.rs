use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
        mut x: [i64; m],
    }
    x.sort();
    let mut same = vec![false; m - 1];
    let mut d = (1..m).map(|i| (x[i] - x[i - 1], i - 1)).collect::<Vec<_>>();
    d.sort();
    let k = if n > m { 0 } else { m - n };
    for i in 0..k {
        same[d[i].1] = true
    }
    // eprintln!("{:?}", x);
    // eprintln!("{:?}", d);
    // eprintln!("{:?}", same);
    let mut s = 0;
    for i in 0..m - 1 {
        if same[i] {
            s += x[i + 1] - x[i];
        }
    }
    println!("{}", s);
}
