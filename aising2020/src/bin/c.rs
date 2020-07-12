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
    }
    let mut f = vec![0; n + 1];
    for x in 1..=n {
        if x * x > n {
            break;
        }
        for y in 1..=n {
            if x * x + y * y + x * y > n {
                break;
            }
            for z in 1..=n {
                let s = x * x + y * y + z * z + x * y + y * z + z * x;
                if s > n {
                    break;
                }
                f[s] += 1;
            }
        }
    }
    for i in 1..=n {
        println!("{}", f[i]);
    }
}
