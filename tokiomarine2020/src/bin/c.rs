use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use segment_tree::ops::Add;
use segment_tree::PointSegment;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use std::iter::repeat;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }
    // for _ in 0..k {
    //     let mut b = vec![0; n];
    //     for i in 0..n {
    //         b[i] += 1;
    //         for j in 1..=a[i] {
    //             if i >= j {
    //                 b[i - j] += 1;
    //             }
    //             if i + j < n {
    //                 b[i + j] += 1;
    //             }
    //         }
    //     }
    //     for i in 0..n {
    //         a[i] = b[i];
    //     }
    // }
    // for i in 0..n {
    //     println!("{}", a[i]);
    // }

    for _ in 0..k {
        let mut tree = PointSegment::build(repeat(0).take(n).collect(), Add);
        for i in 0..n {
            let left = if i > a[i] { i - a[i] } else { 0 };
            let right = if a[i] + i < n { i + a[i] } else { n - 1 };
            // eprintln!("{} {} {}", i, left, right);
            tree.modify(left, right + 1, 1);
        }
        for i in 0..n {
            a[i] = tree.query(i);
        }
        if (0..n).all(|i| a[i] == n) {
            break;
        }
    }
    for i in 0..n {
        println!("{}", a[i]);
    }
}
