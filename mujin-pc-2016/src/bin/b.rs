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
        mut l: [usize; 3],
    }
    l.sort();

    let ro = l[0] + l[1] + l[2];
    if l[0] + l[1] > l[2] && l[0] + l[2] > l[1] && l[1] + l[2] > l[0] {
        println!("{}", PI * ro.pow(2) as f64);
    } else {
        let ri = l[2] - l[1] - l[0];
        println!("{}", PI * (ro.pow(2) - ri.pow(2)) as f64);
    }
}
