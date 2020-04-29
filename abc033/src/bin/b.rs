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
        mut sp: [(String, usize); n],
    }
    sp.sort_by_key(|&(_, pi)| pi);
    sp.reverse();
    let s = sp.iter().map(|&(_, pi)| pi).sum::<usize>();
    if sp[0].1 > s / 2 {
        println!("{}", sp[0].0);
    } else {
        println!("atcoder");
    }
}
