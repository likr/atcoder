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
        s: [String; n],
    }
    let mut count = HashMap::new();
    for si in s.iter() {
        *count.entry(si).or_insert(0) += 1;
    }
    println!("{}", s.iter().max_by_key(|s| count[s]).unwrap());
}
