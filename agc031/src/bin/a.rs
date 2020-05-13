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
        _n: usize,
        s: Chars,
    }
    let s = s
        .into_iter()
        .map(|c| c as usize - 'a' as usize)
        .collect::<Vec<_>>();
    let mut count = vec![1usize; 26];
    for &c in &s {
        count[c] += 1;
    }
    let mut s = 1usize;
    for i in 0..26 {
        s = s * count[i] % M;
    }
    println!("{}", (s + M - 1) % M);
}
