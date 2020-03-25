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
    }
    let mut count = HashMap::new();
    for &ai in &a {
        *count.entry(ai).or_insert(0) += 1;
    }

    let mut s = 0;
    for &c in count.values() {
        if c > 1 {
            s += c - 1;
        }
    }
    if s % 2 == 0 {
        println!("{}", count.len());
    } else {
        println!("{}", count.len() - 1);
    }
}
