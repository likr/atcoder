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
        q: usize,
        ab: [(usize, usize); q],
    }
    for &(mut ai, mut bi) in &ab {
        if ai > bi {
            std::mem::swap(&mut ai, &mut bi);
        }
        if ai == bi {
            println!("{}", ai + bi - 2);
        } else if ai + 1 == bi {
            println!("{}", ai + bi - 3);
        } else {
            let mut c = ((ai as f64).sqrt() * (bi as f64).sqrt()) as usize;
            if c * c == ai * bi {
                c -= 1;
            }

            if c * (c + 1) >= ai * bi {
                println!("{}", 2 * c - 2);
            } else {
                println!("{}", 2 * c - 1);
            }
        }
    }
}
