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
        a: [usize; n],
    }
    if a[0] != 0 {
        println!("0");
        return;
    }

    let mut count = vec![0; n];
    for i in 0..n {
        count[a[i]] += 1;
    }

    let mut p = vec![0; n];
    p[0] = 3;
    let mut used = vec![0; n];
    used[0] = 1;
    for i in 1..n {
        let ai = a[i];
        if ai == 0 {
            if used[ai] > 3 {
                println!("0");
                return;
            }
            p[i] = 3 - used[ai];
        } else {
            if used[ai] > used[ai - 1] {
                println!("0");
                return;
            }
            p[i] = used[ai - 1] - used[ai];
        }
        used[ai] += 1;
    }
    // eprintln!("{:?}", count);
    // eprintln!("{:?}", p);
    // eprintln!("{:?}", used);

    let mut result = 1usize;
    for i in 0..n {
        result = result * p[i] % M;
    }
    println!("{}", result);
}
