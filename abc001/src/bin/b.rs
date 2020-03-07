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
        m: usize,
    }
    if m < 100 {
        println!("00");
    } else if m <= 5000 {
        println!("{:02}", m / 100);
    } else if m <= 30000 {
        println!("{}", m / 1000 + 50);
    } else if m <= 70000 {
        println!("{}", (m / 1000 - 30) / 5 + 80);
    } else {
        println!("89");
    }
}
