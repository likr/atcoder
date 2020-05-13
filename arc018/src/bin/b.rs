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
        xy: [(f64, f64); n],
    }
    let mut result = 0;
    for i in 2..n {
        let (xi, yi) = xy[i];
        for j in 1..i {
            let (xj, yj) = xy[j];
            for k in 0..j {
                let (xk, yk) = xy[k];
                if (xi == xj && xj == xk) || (yi == yj && yj == yk) {
                    continue;
                }
                if xi == xj || xi == xk || xj == xk || yi == yj || yi == yk || yj == yk {
                    result += 1;
                }
            }
        }
    }
    println!("{}", result);
}
