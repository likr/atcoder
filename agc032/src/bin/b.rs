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
    }
    if n % 2 == 0 {
        println!("{}", n * (n - 2) / 2);
        for i in 1..=n {
            for j in i + 1..=n {
                if i + j != n + 1 {
                    println!("{} {}", i, j);
                }
            }
        }
    } else {
        println!("{}", (n - 1) * (n - 1) / 2);
        for i in 1..n {
            for j in i + 1..=n {
                if i + j != n {
                    println!("{} {}", i, j);
                }
            }
        }
    }
}
