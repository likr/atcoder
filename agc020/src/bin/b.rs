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
        k: usize,
        a: [usize; k],
    }
    let mut p_min = 2;
    let mut p_max = 2;
    for i in (0..k).rev() {
        eprintln!("{} {}", p_min, p_max);
        let g_min = p_min / a[i];
        let g_max = p_max / a[i];
        p_max = a[i] * (g_max + 1) - 1;
        p_min = a[i] * g_min;
    }
    println!("{} {}", p_min, p_max);
}
