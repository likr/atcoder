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
        l: usize,
        a: [usize; n],
    }
    let k = (1..n).max_by_key(|&i| a[i - 1] + a[i]).unwrap();
    if a[k - 1] + a[k] < l {
        println!("Impossible");
        return;
    }
    println!("Possible");
    for i in 1..k {
        println!("{}", i);
    }
    for i in (k + 1..n).rev() {
        println!("{}", i);
    }
    println!("{}", k);
}
