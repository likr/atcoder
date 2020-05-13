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
        mut r: [isize; n],
    }
    r.dedup();
    let mut count = 0;
    for i in 1..r.len() - 1 {
        if (r[i - 1] < r[i] && r[i] > r[i + 1]) || (r[i - 1] > r[i] && r[i] < r[i + 1]) {
            count += 1;
        }
    }
    if count > 0 {
        count += 2;
    }
    println!("{}", count);
}
