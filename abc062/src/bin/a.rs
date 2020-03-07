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
        x: Usize1,
        y: Usize1,
    }
    let group = [0, 2, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0];
    if group[x] == group[y] {
        println!("Yes");
    } else {
        println!("No");
    }
}
