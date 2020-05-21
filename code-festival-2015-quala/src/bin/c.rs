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
        t: usize,
        mut ab: [(usize, usize); n],
    }
    let s_a = ab.iter().map(|&(ai, _)| ai).sum::<usize>();
    let s_b = ab.iter().map(|&(_, bi)| bi).sum::<usize>();
    if s_a <= t {
        println!("0");
        return;
    }
    if s_b > t {
        println!("-1");
        return;
    }
    ab.sort_by_key(|&(ai, bi)| ai - bi);
    ab.reverse();
    let mut c = 0;
    for i in 0..n {
        c += ab[i].0 - ab[i].1;
        if s_a - c <= t {
            println!("{}", i + 1);
            return;
        }
    }
}
