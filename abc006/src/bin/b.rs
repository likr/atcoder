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
    if n < 3 {
        println!("0");
    } else if n == 3 {
        println!("1");
    } else {
        let mut a1 = 0;
        let mut a2 = 0;
        let mut a3 = 1;
        for _ in 3..n {
            let t = (a1 + a2 + a3) % 10007;
            a1 = a2;
            a2 = a3;
            a3 = t;
        }
        println!("{}", a3);
    }
}
