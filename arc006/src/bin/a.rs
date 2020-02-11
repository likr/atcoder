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
        e: [usize; 6],
        b: usize,
        l: [usize; 6],
    }
    let e_set = e.iter().collect::<HashSet<_>>();
    let mut count = 0;
    for i in 0..6 {
        if e_set.contains(&l[i]) {
            count += 1;
        }
    }
    if count < 3 {
        println!("0");
    } else if count == 6 {
        println!("1");
    } else {
        if count == 5 {
            for i in 0..6 {
                if !e_set.contains(&l[i]) && l[i] == b {
                    println!("2");
                    return;
                }
            }
            println!("3");
        } else {
            println!("{}", 8 - count);
        }
    }
}
