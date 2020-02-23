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

fn main() {
    input! {
        n: usize,
    }
    let vacant = "Vacant";
    println!("0");
    input! {
        s0: String,
    }
    if s0 == vacant {
        return;
    }
    println!("{}", n - 1);
    input! {
        sn: String,
    }
    if sn == vacant {
        return;
    }
    let mut l = 0;
    let mut r = n - 1;
    while l + 1 < r {
        let m = (l + r) / 2;
        println!("{}", m);
        input! {
            sm: String,
        }
        if sm == vacant {
            return;
        }
        if (m % 2 == 0 && sm == s0) || (m % 2 == 1 && sm == sn) {
            l = m;
        } else {
            r = m;
        }
    }
}
