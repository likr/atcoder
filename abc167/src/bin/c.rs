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
        x: usize,
        ca: [(usize, [usize; m]); n],
    }
    let mut result = INF;
    for i in 1..2usize.pow(n as u32) {
        let mut skills = vec![0; m];
        let mut cost = 0;
        for j in 0..n {
            if i & 1 << j > 0 {
                cost += ca[j].0;
                for k in 0..m {
                    skills[k] += ca[j].1[k];
                }
            }
        }
        if (0..m).all(|k| skills[k] >= x) {
            result = min(result, cost);
        }
    }
    if result == INF {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
