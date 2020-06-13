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
    let floor = [
        "B9", "B8", "B7", "B6", "B5", "B4", "B3", "B2", "B1", "1F", "2F", "3F", "4F", "5F", "6F",
        "7F", "8F", "9F",
    ];
    let a = floor.iter().position(|&u| u == s).unwrap();
    let b = floor.iter().position(|&u| u == t).unwrap();
    println!("{}", max(a, b) - min(a, b));
}
