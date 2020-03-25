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
        h: usize,
        ab: [(usize, usize); n],
    }
    let a = ab.iter().map(|&(ai, _)| ai).collect::<Vec<_>>();
    let mut b = ab.iter().map(|&(_, bi)| bi).collect::<Vec<_>>();
    b.sort();
    b.reverse();
    let a_max = *a.iter().max().unwrap();

    let mut result = 0;
    let mut d = 0;
    for i in 0..n {
        if b[i] > a_max {
            result += 1;
            d += b[i];
            if d >= h {
                println!("{}", result);
                return;
            }
        }
    }
    result += (h - d + a_max - 1) / a_max;
    println!("{}", result);
}
