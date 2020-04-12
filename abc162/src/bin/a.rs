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
        mut n: usize,
    }
    let mut flag = false;
    while n > 0 {
        if n % 10 == 7 {
            flag = true;
        }
        n /= 10;
    }
    if flag {
        println!("Yes");
    } else {
        println!("No");
    }
}
