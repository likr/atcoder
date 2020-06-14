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
        x: usize,
        mut l: [usize; n],
    }
    l.reverse();
    l.push(0);
    l.reverse();
    for i in 1..=n {
        l[i] += l[i - 1];
    }
    eprintln!("{:?}", l);
    println!("{}", l.iter().filter(|&&li| li <= x).count());
}
