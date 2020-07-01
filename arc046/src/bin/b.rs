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
        a: usize,
        b: usize,
    }
    if a == b {
        if n % (a + 1) == 0 {
            println!("Aoki");
        } else {
            println!("Takahashi");
        }
    } else if n <= a {
        println!("Takahashi");
    } else {
        if a < b {
            println!("Aoki");
        } else {
            println!("Takahashi");
        }
    }
}
