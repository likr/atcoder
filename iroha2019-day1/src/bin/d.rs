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
        x: usize,
        y: usize,
        mut a: [usize; n],
    }
    a.sort();
    a.reverse();
    let mut sx = x;
    let mut sy = y;
    for i in 0..n {
        if i % 2 == 0 {
            sx += a[i];
        } else {
            sy += a[i];
        }
    }
    if sx == sy {
        println!("Draw");
    } else if sx > sy {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
