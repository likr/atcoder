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
    let mut result = 0;
    for (x, c) in count.iter() {
        if x <= c {
            result += c - x;
        } else {
            result += c;
        }
    }
    println!("{}", result);
}
