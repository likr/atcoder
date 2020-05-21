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
        d: [usize; n],
    }
    let s = d.iter().sum::<usize>();
    let d_max = d.iter().max().unwrap();
    println!("{}", s);
    if d_max * 2 >= s {
        println!("{}", 2 * d_max - s);
    } else {
        println!("0");
    }
}
