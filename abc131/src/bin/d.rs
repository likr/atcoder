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
        mut ab: [(usize, usize); n],
    }
    ab.sort_by_key(|&(_, bi)| bi);
    let mut t = 0;
    for &(ai, bi) in &ab {
        t += ai;
        if t > bi {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
