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
    let mut factors = HashSet::new();
    for d in 1.. {
        if d * d > n {
            break;
        }
        if n % d == 0 {
            factors.insert(n / d);
            factors.insert(d);
        }
    }
    let mut s = 0;
    for &d in &factors {
        s += d;
    }
    s -= n;
    if s < n {
        println!("Deficient");
    } else if s > n {
        println!("Abundant");
    } else {
        println!("Perfect");
    }
}
