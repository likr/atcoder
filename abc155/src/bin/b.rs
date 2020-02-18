use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
    if a.iter()
        .all(|&ai| ai % 2 == 1 || (ai % 3 == 0 || ai % 5 == 0))
    {
        println!("APPROVED");
    } else {
        println!("DENIED");
    }
}
