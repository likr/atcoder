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
        k: usize,
        s: usize,
    }
    if s == 1000000000 {
        for _ in 0..k {
            println!("{}", s);
        }
        for _ in k..n {
            println!("1");
        }
    } else {
        for _ in 0..k {
            println!("{}", s);
        }
        for _ in k..n {
            println!("1000000000");
        }
    }
}
