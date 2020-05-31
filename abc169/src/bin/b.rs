use num_bigint::*;
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
    if a.iter().any(|&ai| ai == 0) {
        println!("0");
        return;
    }
    let a = a
        .into_iter()
        .map(|ai| ai.to_bigint().unwrap())
        .collect::<Vec<_>>();
    let mut result = 1.to_bigint().unwrap();
    let limit = 10usize.pow(18).to_bigint().unwrap();
    for i in 0..n {
        result *= a[i].clone();
        if result > limit {
            println!("-1");
            return;
        }
    }
    println!("{}", result);
}
