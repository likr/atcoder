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
        m: usize,
        k: usize,
    }
    let n = 2usize.pow(m as u32);
    if k >= n || (m == 1 && k == 1) {
        println!("-1");
        return;
    }
    let mut result = vec![];
    if k == 0 {
        for a in 0..n {
            result.push(a);
            result.push(a);
        }
    } else {
        result.push(k);
        for a in 0..n {
            if a != k {
                result.push(a);
            }
        }
        result.push(k);
        for a in (0..n).rev() {
            if a != k {
                result.push(a);
            }
        }
    }

    for &ai in &result {
        println!("{}", ai);
    }
}
