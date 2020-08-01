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
        l: usize,
        r: usize,
    }
    let mut div = vec![];
    for x in l..=min(r, l + 2019 * 2) {
        div.push(x % 2019);
    }
    let mut result = INF;
    for j in 0..div.len() {
        for i in 0..j {
            result = min(result, div[i] * div[j] % 2019);
        }
    }
    println!("{}", result);
}
