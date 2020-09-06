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
        k: usize,
    }
    let n = 50;
    let mut a = (0..n).collect::<Vec<usize>>();
    for i in 0..n {
        a[i] += k / n;
    }
    for i in 0..k % n {
        for j in 0..n {
            if i == j {
                a[j] += n;
            } else {
                a[j] -= 1;
            }
        }
    }
    println!("{}", n);
    println!(
        "{}",
        a.iter()
            .map(|&ai| format!("{}", ai))
            .collect::<Vec<_>>()
            .join(" ")
    );
}
