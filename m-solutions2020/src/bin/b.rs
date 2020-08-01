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
        a: usize,
        b: usize,
        c: usize,
        k: usize,
    }
    let mut ok = false;
    for x in 0..=k {
        for y in 0..=k {
            for z in 0..=k {
                if x + y + z <= k {
                    if a * 2usize.pow(x as u32) < b * 2usize.pow(y as u32)
                        && b * 2usize.pow(y as u32) < c * 2usize.pow(z as u32)
                    {
                        ok = true;
                    }
                }
            }
        }
    }
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
