use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

// fn s(n: usize) -> usize {
//     let mut ret = 0;
//     let mut m = n;
//     while m > 0 {
//         ret += m % 10;
//         m /= 10;
//     }
//     ret
// }

fn main() {
    input! {
        mut k: usize,
    }
    // let a = (1..=k)
    //     .map(|n| {
    //         let m = s(n);
    //         let s = n as f64 / m as f64;
    //         (n, s)
    //     })
    //     .collect::<Vec<_>>();
    // for i in 0..a.len() {
    //     let (n, si) = a[i];
    //     if (i + 1..a.len()).all(|j| si <= a[j].1) {
    //         println!("{}", n);
    //     }
    // }
    let mut base = 0;
    let mut d = 1;
    'finish: loop {
        for i in 1..=9 {
            println!("{}", i);
        }
    }
}
