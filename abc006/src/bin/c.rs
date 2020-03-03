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
        m: usize,
    }
    if 2 * n > m {
        println!("-1 -1 -1");
        return;
    }
    for c in 0..=m - 2 * n {
        if (m - 2 * n - c) % 2 == 1 {
            continue;
        }
        let b = (m - 2 * n - c) / 2;
        if b + c > n {
            continue;
        }
        let a = n - b - c;
        eprintln!("{} {}", a + b + c, 2 * a + 4 * b + 3 * c);
        println!("{} {} {}", a, c, b);
        return;
    }
    eprintln!("not found");
    println!("-1 -1 -1");
}
