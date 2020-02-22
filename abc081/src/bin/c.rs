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
        k: usize,
        a: [usize; n],
    }
    let mut count = HashMap::new();
    for &ai in &a {
        *count.entry(ai).or_insert(0) += 1;
    }
    let mut count = count.values().map(|v| v).collect::<Vec<_>>();
    count.sort();
    let mut result = 0;
    if count.len() > k {
        for i in 0..count.len() - k {
            result += count[i];
        }
    }
    println!("{}", result);
}
