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
const INF: isize = std::isize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        x: isize,
        y: isize,
    }
    let mut result = INF;
    if y >= x {
        result = min(result, y - x);
    }
    if y >= -x {
        result = min(result, y + x + 1);
    }
    if -y >= x {
        result = min(result, -y - x + 1);
    }
    if -y >= -x {
        result = min(result, -y + x + 2);
    }
    println!("{}", result);
}
