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
        s: String,
        t: String,
    }
    if s == t {
        println!("same");
    } else if s.to_lowercase() == t.to_lowercase() {
        println!("case-insensitive");
    } else {
        println!("different");
    }
}
