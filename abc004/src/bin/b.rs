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
        c: [[char; 4]; 4],
    }
    for i in 0..4 {
        let mut chars = vec![];
        for j in 0..4 {
            chars.push(format!("{}", c[3 - i][3 - j]));
        }
        println!("{}", chars.join(" "));
    }
}
