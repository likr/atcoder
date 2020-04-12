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
        a: [usize; n],
    }
    let s = a.iter().sum::<usize>();
    let min_count = s as f64 / (4. * m as f64);
    if a.iter().filter(|&&ai| ai as f64 >= min_count).count() >= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
