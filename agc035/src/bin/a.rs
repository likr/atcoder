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
        a: [usize; n],
    }
    let mut a_max = *a.iter().max().unwrap();
    let mut m = 0;
    while a_max > 0 {
        a_max /= 2;
        m += 1;
    }
    let mut count = vec![0; m];
    for &ai in &a {
        for j in 0..m {
            if ai & 1 << j > 0 {
                count[j] += 1;
            }
        }
    }
    if count.iter().all(|&c| c % 2 == 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
