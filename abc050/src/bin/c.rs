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
    let mut count = vec![0; n];
    for &ai in &a {
        count[ai] += 1;
    }
    if n % 2 == 0 {
        if (1..n).step_by(2).any(|i| count[i] != 2) {
            println!("0");
            return;
        }
    } else {
        if count[0] != 1 || (2..n).step_by(2).any(|i| count[i] != 2) {
            println!("0");
            return;
        }
    }

    let mut result = 1;
    for _ in 0..n / 2 {
        result = result * 2 % M;
    }
    println!("{}", result);
}
