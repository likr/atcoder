#![feature(map_first_last)]
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
        q: usize,
    }
    let mut left = BTreeSet::new();
    left.insert((-INF, 0));
    let mut right = BTreeSet::new();
    right.insert((INF, 1));
    let mut y = 0;
    for i in 1..=q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                a: isize,
                b: isize,
            }
            y += b;
            let (x0, k0) = left.pop_last().unwrap();
            let (x1, k1) = right.pop_first().unwrap();
            if a < x0 {
                y += x0 - a;
                left.insert((a, 2 * i));
                left.insert((a, 2 * i + 1));
                right.insert((x0, k0));
                right.insert((x1, k1));
            } else if x1 < a {
                y += a - x1;
                left.insert((x0, k0));
                left.insert((x1, k1));
                right.insert((a, 2 * i));
                right.insert((a, 2 * i + 1));
            } else {
                left.insert((x0, k0));
                left.insert((a, 2 * i));
                right.insert((a, 2 * i + 1));
                right.insert((x1, k1));
            }
        // eprintln!("{:?}", left);
        // eprintln!("{:?}", right);
        } else {
            let (x, _) = left.last().unwrap();
            println!("{} {}", x, y);
        }
    }
}
