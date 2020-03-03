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
        m: usize,
        sc: [(usize, usize); m],
    }
    if n == 1
        && sc
            .iter()
            .all(|&(si, ci)| 0 / 10usize.pow((n - si) as u32) % 10 == ci)
    {
        println!("{}", 0);
        return;
    }
    for x in 10usize.pow(n as u32 - 1)..10usize.pow(n as u32) {
        if sc
            .iter()
            .all(|&(si, ci)| x / 10usize.pow((n - si) as u32) % 10 == ci)
        {
            println!("{}", x);
            return;
        }
    }
    println!("-1");
}
